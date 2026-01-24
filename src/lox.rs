use std::env;
use std::process::ExitCode;

mod run_file;
mod run_prompt;
mod run;

pub fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let number_of_args: usize = args.len();

    if number_of_args > 2 {
        println!("Usage: cargo run [script]");
        return ExitCode::from(64);
    } else if number_of_args == 2 {
        if let Err(e) = run_file::run_file(&args[1]) {
            eprintln!("Error reading file: {}", e);
            return ExitCode::from(74);
        }
    } else {
        println!("running prompt")
    }

    return ExitCode::SUCCESS;
}
