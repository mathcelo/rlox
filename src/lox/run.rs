use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub(super) fn run(line: &str) {
    println!("Running line: {}", line)
}

fn error(line_number: usize, message: &str) {
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
