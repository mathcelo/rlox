use std::env;
use std::process::ExitCode;

mod config;
mod lexer;
mod runner;
mod syntax;

pub fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let number_of_args: usize = args.len();

    if number_of_args > 2 {
        println!("Usage: cargo run [script]");
        return config::exit_codes::usage_error();
    } else if number_of_args == 2 {
        if let Err(e) = runner::run_file::run_file(&args[1]) {
            eprintln!("Error reading file: {}", e);
            return config::exit_codes::io_error();
        }
        if runner::run::had_error() {
            return config::exit_codes::data_error();
        }
    } else {
        runner::run_prompt::run_prompt();
        println!("running prompt")
    }

    return ExitCode::SUCCESS;
}
