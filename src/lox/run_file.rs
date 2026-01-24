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

    let mut source = String::new();
    for line_result in reader.lines() {
        let line = line_result?;
        source.push_str(&line);
        source.push('\n');
    }

    run::run(&source);

    Ok(())
}
