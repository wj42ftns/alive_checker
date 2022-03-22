use cmd_lib::run_cmd;

fn show_notification(err_message: &str) {
    run_cmd! (
        notify-send -t 0 $err_message;
    )
    .expect("Error: Could not show notification with error!");
}

pub fn handle(err_message: &str) {
    let prefix = "Error: ";
    let message_with_prefix = format!("{}{}", prefix, err_message);
    show_notification(&message_with_prefix);
    panic!(
        "Finishing script because of catching an error: '{}'",
        message_with_prefix
    );
}

pub fn handle_silently(err_message: &str) {
    show_notification(err_message);
    std::process::exit(exitcode::OK);
}
