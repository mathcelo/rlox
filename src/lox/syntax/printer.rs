use super::ast::Expr;
use crate::lox::lexer::token::Literal;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right } => {
                Self::parenthesize(&operator.lexeme, &[left.as_ref(), right.as_ref()])
            }
            Expr::Unary { operator, right } => {
                Self::parenthesize(&operator.lexeme, &[right.as_ref()])
            }
            Expr::Literal { value } => {
                if let Some(literal) = value {
                    match literal {
                        Literal::String(s) => s.clone(),
                        Literal::Number(n) => n.to_string(),
                        Literal::Boolean(b) => b.to_string(),
                        Literal::Nil => "nil".to_string(),
                    }
                } else {
                    "nil".to_string()
                }
            }
            Expr::Grouping { expression } => {
                Self::parenthesize("group", &[expression.as_ref()])
            }
        }
    }

    fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&Self::print(expr));
        }
        builder.push(')');
        builder
    }
}