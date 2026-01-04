use crate::lexer::token::{Token, TokenType};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct ExpectedTokenError {
    actual: Option<Token>,
    expected: TokenType,
}

impl ExpectedTokenError {
    pub fn new(
        actual: Option<Token>,
        expected: TokenType,
    ) -> Self {
        Self {
            actual,
            expected,
        }
    }
}

impl std::fmt::Display for ExpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.actual {
            None => write!(f, "{} expected",
                           self.expected,
            ),
            Some(token) => write!(f, "{} expected but got {}",
                                  self.expected,
                                  token,
            ),
        }
    }
}
