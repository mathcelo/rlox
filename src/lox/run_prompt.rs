use std::io::{self, Write};

use super::run;

pub fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Force it to display immediately

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            break;
        }
        run::run(trimmed_input);
        run::clear_error();
    }
}
