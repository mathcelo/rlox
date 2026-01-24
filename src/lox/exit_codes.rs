use std::process::ExitCode;
use once_cell::sync::Lazy;

const EXIT_CODES_TOML: &str = include_str!("../../exit_codes.toml");

static EXIT_CODES: Lazy<toml::Value> = Lazy::new(|| {
    EXIT_CODES_TOML.parse().expect("Failed to parse exit_codes.toml")
});

pub fn usage_error() -> ExitCode {
    ExitCode::from(
        EXIT_CODES["exit_codes"]["usage_error"]
            .as_integer()
            .expect("usage_error must be an integer") as u8,
    )
}

pub fn data_error() -> ExitCode {
    ExitCode::from(
        EXIT_CODES["exit_codes"]["data_error"]
            .as_integer()
            .expect("data_error must be an integer") as u8,
    )
}

pub fn io_error() -> ExitCode {
    ExitCode::from(
        EXIT_CODES["exit_codes"]["io_error"]
            .as_integer()
            .expect("io_error must be an integer") as u8,
    )
}
