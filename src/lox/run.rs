use std::sync::atomic::{AtomicBool, Ordering};

use super::scanner::Scanner;

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub(super) fn run(source: &str) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    
    // For now, just print the tokens
    for token in tokens {
        println!("{}", token);
    }
}

pub(super) fn error(line_number: usize, message: &str) {
    report(line_number, "", message);
}

fn report(line_number: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line_number, location, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}

pub(super) fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

pub(super) fn clear_error() {
    HAD_ERROR.store(false, Ordering::Relaxed);
}
