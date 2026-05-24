use std::env;
use std::fs;
use std::process;
use std::process::Command;

use flutter::FlutterSdk;
use inquire::Select;
use inquire::Text;

mod flutter;
mod preflight;

fn main() {
    // first try to access commandline argument or fallback to cwd
    let target_path = env::args().nth(1).unwrap_or_else(|| {
        env::current_dir()
            .expect("error: no path specified failed to get cwd as fallback")
            .to_str()
            .unwrap()
            .to_string()
    });

    let resolved_target_path =
        fs::canonicalize(target_path).expect("error: failed to resolve target path");

    println!("cwd: {}", resolved_target_path.display());

    // check for flutter in sys-path and fallback to user-prompt if not successful
    let flutter_path = preflight::check_flutter_binary().unwrap_or_else(|| {
        let manual_flutter_path = Text::new("specify the flutter path:")
            .prompt()
            .expect("error: failed to read flutter path from prompt");

        let resolved =
            fs::canonicalize(&manual_flutter_path).expect("error: failed to resolve flutter path");

        if !resolved.is_file() {
            eprintln!("error: provided flutter path is not a file");
            process::exit(1);
        }

        resolved
    });

    preflight::check_pubspec_file(&resolved_target_path);

    let flutter = FlutterSdk::new(&flutter_path);
    let devices = flutter
        .get_devices()
        .expect("err: retrieving flutter devices");

    if devices.is_empty() {
        println!("there are no flutter devices to operate on");
        process::exit(2);
    }

    let selected_device = Select::new("Select device to run on", devices)
        .prompt()
        .expect("error: while selecting device");

    Command::new(&flutter_path)
        .args(["run", "-d", &selected_device])
        .status()
        .expect("error: failed to spawn flutter run");
}
