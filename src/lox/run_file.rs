use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

use super::run;

pub fn run_file(path: &str) -> std::io::Result<()> {
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

    let reader = BufReader::new(lox_file);

    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors for each line

        run::run(&line);
    }

    Ok(())
}
