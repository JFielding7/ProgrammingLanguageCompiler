use thiserror::Error;
use crate::error_util::ErrorLocation;
use crate::lexer::token::{Token, TokenType};

#[derive(Debug, Error)]
pub struct ExpectedTokenError {
    actual: Option<Token>,
    expected: TokenType,
    error_location: ErrorLocation,
}

impl ExpectedTokenError {
    pub fn new(
        actual: Option<Token>,
        expected: TokenType,
        error_location: ErrorLocation
    ) -> Self {
        Self {
            actual,
            expected,
            error_location,
        }
    }
}

impl std::fmt::Display for ExpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.actual {
            None => write!(f, "{} expected\n{}",
                           self.expected,
                           self.error_location
            ),
            Some(token) => write!(f, "{} expected but got {}\n{}",
                                  self.expected,
                                  token,
                                  self.error_location
            ),
        }
    }
}
