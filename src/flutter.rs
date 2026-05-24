use std::{
    path::{Path, PathBuf},
    process::Command,
};

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

    pub fn get_devices(&self) {
        let output = Command::new(self.path.clone())
            .arg("devices")
            .output()
            .expect("err: flutter devices failed!");

        if output.status.success() {
            let output = String::from_utf8(output.stdout).unwrap();
            self.extract_devices(&output);
        }
    }

    fn extract_devices(&self, output: &str) -> Vec<String> {
        const INDICATOR: &str = "•";

        let start_index = output
            .find("connected devices:")
            .expect("err: no start indicator found");

        let last_index = output
            .rfind(INDICATOR)
            .expect("err: no end indicator found");

        assert!(
            start_index < last_index,
            "err: invalid output of flutter devices"
        );

        let devices_output = &output[start_index..last_index];
        let device_lines: Vec<String> = devices_output.split('\n').map(String::from).collect();

        let mut devices: Vec<String> = vec![];

        for line in device_lines {
            let dot_indices: Vec<usize> = line.match_indices(INDICATOR).map(|(i, _)| i).collect();

            let Some(&first_dot_index) = dot_indices.get(0) else {
                continue;
            };

            let Some(&second_dot_index) = dot_indices.get(1) else {
                continue;
            };

            let Some(device) = line.get(first_dot_index..second_dot_index) else {
                continue;
            };

            devices.push(String::from(device.trim()));
        }

        return devices;
    }
}
