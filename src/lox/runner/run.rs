use crate::lox::error;
use crate::lox::lexer::scanner::Scanner;
use crate::lox::syntax::parser::Parser;
use crate::lox::syntax::printer::AstPrinter;

pub(super) fn run(source: &str) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let expression = parser.parse();

    // Stop if there was a syntax error.
    if error::had_error() {
        return;
    }

    if let Some(expr) = expression {
        println!("{}", AstPrinter::print(&expr));
    }
}
