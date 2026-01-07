use crate::error::spanned_error::SpannedError;

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("Error: Unrecognized token: {0}")]
    InvalidToken(String),

    #[error("Error: Unaligned Indent: Indent size {0} is not a multiple of 4")]
    UnalignedIndent(usize),
}


pub type LexerResult<T> = Result<T, SpannedError>;
