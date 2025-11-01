use std::fs;

pub fn run_file(file_path: &str) {
    println!("File: {file_path}");
    let contents: String = fs::read_to_string(file_path).expect("Error");
    println!("{contents}");
}

pub fn run_prompt() {
    unimplemented!("run_prompt is not yet implemented");
}
