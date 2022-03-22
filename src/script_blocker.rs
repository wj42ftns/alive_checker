use crate::error;
use cmd_lib::run_cmd;
use std::fs::File;
use std::path::Path;

const FAILURE_SYSTEM: &str = "FailureSystem";

pub fn handle_wrong_answer() {
    let mut failure_system_message: String = String::from(FAILURE_SYSTEM);
    for _ in 0..10 {
        failure_system_message = format!("{} {}", failure_system_message, FAILURE_SYSTEM);
    }
    if run_cmd! {
        echo $failure_system_message
    }
    .is_err()
    {
        error::handle(&format!("Could not print '{}' in console!", FAILURE_SYSTEM));
    };

    File::create(&get_path_to_alive_checker_blocker())
        .expect("Error: Could not create '.alive_checker_blocker' file!");
    error::handle_silently("A wrong answer to asked numbers. Alive_checker script is blocked.");
}

pub fn get_has_script_blocker_file() -> bool {
    Path::new(&get_path_to_alive_checker_blocker()).exists()
}

fn get_path_to_alive_checker_blocker() -> String {
    let mut dir =
        std::env::current_exe().expect("Error: Could not get current binary absolute path!");
    dir.pop();
    dir.push(".alive_checker_blocker");

    dir.display().to_string()
}
