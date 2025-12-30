use crate::lexer::error::invalid_token::InvalidTokenError;
use crate::lexer::error::unaligned_indent::UnalignedIndentError;

pub mod invalid_token;
pub mod unaligned_indent;

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("Error: {0}")]
    InvalidToken(#[from] InvalidTokenError),

    #[error("Error: {0}")]
    UnalignedIndent(#[from] UnalignedIndentError),
}

pub type LexerResult<T> = Result<T, LexerError>;