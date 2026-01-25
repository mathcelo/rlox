// Parser for converting tokens into an Abstract Syntax Tree (AST)
// This will be implemented following the Crafting Interpreters book

use super::ast::Expr;
use crate::lox::lexer::token::{Literal, Token};
use crate::lox::lexer::token_type::TokenType;
use crate::lox::error;

#[derive(Debug, Clone)]
pub struct ParseError;

pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
  had_error: bool,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      current: 0,
      had_error: false,
    }
  }

  pub fn parse(&mut self) -> Option<Expr> {
    // Reset error state
    self.had_error = false;

    let expr: Expr = self.parse_expression();
    if self.had_error { None } else { Some(expr) }
  }

  fn parse_expression(&mut self) -> Expr {
    self.parse_equality()
  }

  fn parse_equality(&mut self) -> Expr {
    let mut expr = self.parse_comparison();
    while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
      let operator = self.previous();
      let right = self.parse_comparison();
      expr = Expr::Binary {
        left: Box::new(expr),
        operator,
        right: Box::new(right),
      };
    }
    expr
  }

  fn parse_comparison(&mut self) -> Expr {
    let mut expr = self.parse_term();

    while self.matches(&[
      TokenType::Greater,
      TokenType::GreaterEqual,
      TokenType::Less,
      TokenType::LessEqual,
    ]) {
      let operator = self.previous();
      let right = self.parse_term();
      expr = Expr::Binary {
        left: Box::new(expr),
        operator,
        right: Box::new(right),
      };
    }
    expr
  }

  fn parse_term(&mut self) -> Expr {
    let mut expr = self.parse_factor();
    while self.matches(&[TokenType::Minus, TokenType::Plus]) {
      let operator = self.previous();
      let right = self.parse_factor();
      expr = Expr::Binary {
        left: Box::new(expr),
        operator,
        right: Box::new(right),
      };
    }
    expr
  }

  fn parse_factor(&mut self) -> Expr {
    let mut expr = self.parse_unary();
    while self.matches(&[TokenType::Slash, TokenType::Star]) {
      let operator = self.previous();
      let right = self.parse_unary();
      expr = Expr::Binary {
        left: Box::new(expr),
        operator,
        right: Box::new(right),
      };
    }
    expr
  }

  fn parse_unary(&mut self) -> Expr {
    if self.matches(&[TokenType::Bang, TokenType::Minus]) {
      let operator = self.previous();
      let right = self.parse_unary();
      return Expr::Unary {
        operator,
        right: Box::new(right),
      };
    }
    self.parse_primary()
  }

  fn parse_primary(&mut self) -> Expr {
    if self.matches(&[TokenType::False]) {
      return Expr::Literal {
        value: Some(Literal::Boolean(false)),
      };
    }
    if self.matches(&[TokenType::True]) {
      return Expr::Literal {
        value: Some(Literal::Boolean(true)),
      };
    }
    if self.matches(&[TokenType::Nil]) {
      return Expr::Literal {
        value: Some(Literal::Nil),
      };
    }
    if self.matches(&[TokenType::Number, TokenType::String]) {
      if let Some(literal) = self.previous().literal.clone() {
        return Expr::Literal {
          value: Some(literal),
        };
      }
    }
    if self.matches(&[TokenType::LeftParen]) {
      let expr = self.parse_equality();
      self.consume(TokenType::RightParen, "Expect ')' after expression.");
      return Expr::Grouping {
        expression: Box::new(expr),
      };
    }
    let token = self.peek().clone();
    let _ = self.error(&token, "Expect expression.");
    // Return a dummy expression to allow parsing to continue (error recovery)
    Expr::Literal {
      value: Some(Literal::Nil),
    }
  }

  fn consume(&mut self, token_type: TokenType, message: &str) {
    if self.check(token_type) {
      self.advance();
      return;
    }
    let token = self.peek().clone();
    let _ = self.error(&token, message);
  }

  fn error(&mut self, token: &Token, message: &str) -> ParseError {
    error::error_token(token, message);
    self.had_error = true;
    ParseError
  }


  // Helper methods
  fn previous(&self) -> Token {
    self.tokens[self.current - 1].clone()
  }

  fn peek(&self) -> &Token {
    &self.tokens[self.current]
  }

  fn check(&self, token_type: TokenType) -> bool {
    if self.is_at_end() {
      return false;
    }
    self.peek().token_type == token_type
  }

  fn is_at_end(&self) -> bool {
    self.peek().token_type == TokenType::Eof
  }

  fn advance(&mut self) -> Token {
    if !self.is_at_end() {
      self.current += 1;
    }
    self.previous()
  }

  fn matches(&mut self, token_types: &[TokenType]) -> bool {
    for token_type in token_types {
      if self.check(*token_type) {
        self.advance();
        return true;
      }
    }
    false
  }
}