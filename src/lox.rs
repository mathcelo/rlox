use std::env;
use std::process::ExitCode;

mod exit_codes;
mod run;
mod run_file;
mod run_prompt;

pub fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let number_of_args: usize = args.len();

    if number_of_args > 2 {
        println!("Usage: cargo run [script]");
        return exit_codes::usage_error();
    } else if number_of_args == 2 {
        if let Err(e) = run_file::run_file(&args[1]) {
            eprintln!("Error reading file: {}", e);
            return exit_codes::io_error();
        }
        if run::had_error() {
            return exit_codes::data_error();
        }
    } else {
        run_prompt::run_prompt();
        println!("running prompt")
    }

    return ExitCode::SUCCESS;
}
