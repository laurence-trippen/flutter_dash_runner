use std::{
    fmt, io,
    path::{Path, PathBuf},
    process::Command,
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum FlutterError {
    CommandFailed(io::Error),
    NonZeroExit { code: Option<i32>, stderr: String },
    InvalidUtf8(FromUtf8Error),
    UnexpectedOutput(&'static str),
}

impl fmt::Display for FlutterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandFailed(e) => write!(f, "couldn't run flutter binary: {e}"),
            Self::NonZeroExit { code, stderr } => {
                write!(f, "flutter ended with code {code:?}: {stderr}")
            }
            Self::InvalidUtf8(e) => write!(f, "invalid utf-8 output in flutter: {e}"),
            Self::UnexpectedOutput(msg) => write!(f, "un-expected output format: {msg}"),
        }
    }
}

impl std::error::Error for FlutterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CommandFailed(e) => Some(e),
            Self::InvalidUtf8(e) => Some(e),
            _ => None,
        }
    }
}

// with From-Impls the `?` works automagically:
impl From<io::Error> for FlutterError {
    fn from(e: io::Error) -> Self { Self::CommandFailed(e) }
}

impl From<FromUtf8Error> for FlutterError {
    fn from(e: FromUtf8Error) -> Self { Self::InvalidUtf8(e) }
}

#[derive(Debug, Clone)]
pub struct FlutterSdk {
    path: PathBuf,
}

impl FlutterSdk {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn get_devices(&self) -> Result<Vec<String>, FlutterError> {
        let output = Command::new(&self.path)
            .arg("devices")
            .output()?;

        if !output.status.success() {
            return Err(FlutterError::NonZeroExit {
                code: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }

        let stdout = String::from_utf8(output.stdout)?;  // FromUtf8Error to FlutterError
        self.extract_devices(&stdout)
    }

    fn extract_devices(&self, output: &str) -> Result<Vec<String>, FlutterError> {
        const INDICATOR: &str = "•";

        let start_index = output
            .find("connected devices:")
            .ok_or(FlutterError::UnexpectedOutput("kein Start-Indikator"))?;

        let last_index = output
            .rfind(INDICATOR)
            .ok_or(FlutterError::UnexpectedOutput("kein End-Indikator"))?;

        if start_index >= last_index {
            return Err(FlutterError::UnexpectedOutput(
                "Start liegt hinter Ende",
            ));
        }

        let devices_output = &output[start_index..last_index];
        let mut devices: Vec<String> = Vec::new();

        for line in devices_output.lines() {
            let dot_indices: Vec<usize> =
                line.match_indices(INDICATOR).map(|(i, _)| i).collect();

            let [first, second, ..] = dot_indices.as_slice() else {
                continue;
            };

            let first = *first;
            let second = *second;

            let Some(device) = line.get(first..second) else {
                continue;
            };

            devices.push(device.replace(INDICATOR, "").trim().to_string());
        }

        Ok(devices)
    }
}
