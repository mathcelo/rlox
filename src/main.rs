use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let number_of_args: usize = args.len();

    if number_of_args > 2 {
        println!("Usage: cargo run [script]");
        return ExitCode::from(64);
    } else if number_of_args == 2 {
        println!("running file {}", args[1])
    } else {
        println!("running prompt")
    }

    return ExitCode::SUCCESS;
}
