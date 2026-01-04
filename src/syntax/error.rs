use crate::source::source_span::SourceSpan;
use crate::lexer::error::LexerErrorType;
use crate::lexer::token::{Token, TokenType};
use crate::error::spanned_error::{SpannableError, SpannedError};
use crate::syntax::error::expected_token::ExpectedTokenError;
use crate::syntax::error::SyntaxErrorType::{ExpectedToken, UnmatchedParen};
use crate::syntax::error::unmatched_paren::UnmatchedParenError;

pub mod expected_token;
pub mod unmatched_paren;

#[derive(thiserror::Error, Debug)]
pub enum SyntaxErrorType {

    #[error("Error: {0}")]
    ExpectedToken(#[from] ExpectedTokenError),

    #[error("Error: {0}")]
    UnmatchedParen(#[from] UnmatchedParenError),

    #[error("Error: Invalid Expression")]
    InvalidExpression,

    #[error("Error: Line indented too far in")]
    IndentTooLarge,
}

impl SyntaxErrorType {
    pub fn expected_token(actual: Option<Token>, expected: TokenType) -> Self {
        ExpectedToken(ExpectedTokenError::new(actual, expected))
    }

    pub fn unmatched_paren(paren_type: TokenType) -> Self {
        UnmatchedParen(UnmatchedParenError::new(paren_type))
    }
}

impl SpannableError for SyntaxErrorType {}

pub type SyntaxError = SpannedError<SyntaxErrorType>;
pub type SyntaxResult<T> = Result<T, SyntaxError>;
