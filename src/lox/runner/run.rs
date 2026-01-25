use std::sync::atomic::{AtomicBool, Ordering};

use crate::lox::lexer::scanner::Scanner;
use crate::lox::lexer::token::Token;
use crate::lox::lexer::token_type::TokenType;
use crate::lox::syntax::parser::Parser;
use crate::lox::syntax::printer::AstPrinter;

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub(super) fn run(source: &str) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let expression = parser.parse();

    // Stop if there was a syntax error.
    if had_error() {
        return;
    }

    if let Some(expr) = expression {
        println!("{}", AstPrinter::print(&expr));
    }
}

pub(crate) fn error(line_number: usize, message: &str) {
    report(line_number, "", message);
}

pub(crate) fn report(line_number: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line_number, location, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}

pub(crate) fn error_token(token: &Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

pub(crate) fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

pub(super) fn clear_error() {
    HAD_ERROR.store(false, Ordering::Relaxed);
}
