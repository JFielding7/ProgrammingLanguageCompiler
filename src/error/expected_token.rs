use crate::error::compiler_error::{CompilerError, Result};
use crate::error::error_info::ErrorInfo;
use crate::lexer::token::{Token, TokenType};

#[derive(Debug)]
pub struct ExpectedTokenError {
    actual: Option<Token>,
    expected: TokenType,
    pub error_info: ErrorInfo,
}

impl ExpectedTokenError {
    pub fn new(
        actual: Option<Token>,
        expected: TokenType,
        error_info: ErrorInfo
    ) -> Self {
        Self {
            actual,
            expected,
            error_info,
        }
    }
}

impl std::fmt::Display for ExpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.actual {
            None => write!(f, "{} expected", self.expected),
            Some(token) => write!(f, "expected {} but got {}", self.expected, token),
        }
    }
}

impl From<ExpectedTokenError> for Result<Token> {
    fn from(expected: ExpectedTokenError) -> Self {
        Err(CompilerError::ExpectedToken(expected))
    }
}
