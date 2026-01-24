use std::fs::File;
use std::io::ErrorKind;

pub fn run_file(path: &str) {
    let lox_file_result = File::open(path);

    let lox_file = match lox_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                panic!("File not found: {error:?}");
            }
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
