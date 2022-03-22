use std::io::Write;

pub fn ask_numbers() -> bool {
    let mut line = String::new();
    print!(">");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read the input line!");

    line.trim() == "4 8 15 16 23 42"
}
