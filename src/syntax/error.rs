use crate::error::spanned_error::SpannedError;
use crate::lexer::token::TokenType;

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {

    #[error("Error: {0} expected")]
    ExpectedToken(TokenType),

    #[error("Error: Unmatched {0}")]
    UnmatchedGroupOpening(TokenType),

    #[error("Error: Invalid Expression")]
    InvalidExpression,

    #[error("Error: Line indented too far in")]
    IndentTooLarge,
}

pub type SyntaxResult<T> = Result<T, SpannedError>;
