use std::env;
use std::path::PathBuf;
use std::process;

pub fn check_flutter_binary() -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    let binary_name = if cfg!(windows) {
        "flutter.bat"
    } else {
        "flutter"
    };

    // split path is crucial here to split by OS delimeter
    env::split_paths(&path)
        .map(|dir| dir.join(binary_name))
        .find(|full_path| full_path.is_file())
}

pub fn check_pubspec_file(path: &PathBuf) {
    let pubspec_yml_path = path.join("pubspec.yml");
    let pubspec_yaml_path = path.join("pubspec.yaml");

    if !pubspec_yml_path.exists() && !pubspec_yaml_path.exists() {
        eprintln!("error: there is no pubspec file");
        process::exit(1);
    }
}
