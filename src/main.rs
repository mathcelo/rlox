use std::env;
use std::process;

mod lox;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let number_of_arguments: usize = arguments.len();

    if number_of_arguments > 3 {
        eprintln!("Usage: cargo run main [script]");
        process::exit(64);
    } else if number_of_arguments == 3 {
        lox::run_file("hello there!");
    } else {
        lox::run_prompt();
    }
}
