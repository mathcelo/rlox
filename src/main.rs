use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let number_of_args: usize = args.len();

    if number_of_args > 2 {
        println!("Usage: cargo run [script]")
    } else if number_of_args == 2 {
        println!("running file {}", args[1])
    } else {
        println!("running prompt")
    }
}
