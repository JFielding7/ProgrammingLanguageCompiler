use crate::error_util::ErrorLocation;
use crate::syntax::error::expected_token::ExpectedTokenError;
use crate::syntax::error::unmatched_paren::UnmatchedParenError;

pub mod expected_token;
pub mod unmatched_paren;

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {

    #[error("Error: {0}")]
    ExpectedToken(#[from] ExpectedTokenError),

    #[error("Error: {0}")]
    UnmatchedParen(#[from] UnmatchedParenError),

    #[error("Error: {0}")]
    InvalidExpression(ErrorLocation),
}

pub type SyntaxResult<T> = Result<T, SyntaxError>;
