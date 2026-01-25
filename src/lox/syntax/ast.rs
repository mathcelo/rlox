use crate::lox::lexer::token::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: Option<Literal>,
    },
    Grouping {
        expression: Box<Expr>,
    },
}

