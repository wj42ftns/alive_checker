use cmd_lib::run_cmd;
use std::env::args;

fn get_path_to_alive_script_from_arg() -> String {
    args()
        .nth(1)
        .expect("Error: Could not found a path to an alive_script in argument!")
}

pub fn run(current_date: String) {
    let path_to_alive_script_from_arg = get_path_to_alive_script_from_arg();
    run_cmd!($path_to_alive_script_from_arg $current_date)
        .expect("Error: Could not run alive_script!");
    run_cmd! (notify-send -t 5000 "Success!")
        .expect("Error: Could not show notification with 'Success'!");
}
