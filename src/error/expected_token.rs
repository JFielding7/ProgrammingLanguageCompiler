use crate::error::compiler_error::{CompilerError, Result};
use crate::error::error_info::ErrorInfo;
use crate::lexer::token::{Token, TokenType};

#[derive(Debug)]
pub struct ExpectedToken {
    actual: Option<Token>,
    expected: TokenType,
    error_info: Option<ErrorInfo>,
}

impl ExpectedToken {
    pub fn new(actual: Option<Token>, expected: TokenType) -> Self {
        Self {
            actual,
            expected,
            error_info: None,
        }
    }

    pub fn attach_error_info(&mut self, error_info: ErrorInfo) {
        self.error_info = Some(error_info);
    }
}

impl From<ExpectedToken> for Result<Token> {
    fn from(expected: ExpectedToken) -> Self {
        Err(CompilerError::ExpectedToken(expected))
    }
}
