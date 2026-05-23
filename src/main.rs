use std::env;
use std::fs;
use std::process;

use inquire::Text;

mod preflight;
mod flutter;

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

    println!("flutter: {}", flutter_path.display());

    preflight::check_pubspec_file(&resolved_target_path);
}
