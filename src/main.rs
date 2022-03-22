mod alive_script;
mod date;
mod error;
mod script_blocker;
mod script_guardian;

fn main() {
    if script_blocker::get_has_script_blocker_file() {
        std::process::exit(exitcode::OK);
    }

    let is_right_answer: bool = script_guardian::ask_numbers();
    if !is_right_answer {
        return script_blocker::handle_wrong_answer();
    }

    let current_date: String = date::current();
    alive_script::run(current_date);
}
