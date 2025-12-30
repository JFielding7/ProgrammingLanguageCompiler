use thiserror::Error;
use crate::lexer::error::LexerError;
use crate::syntax::error::SyntaxError;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Error: No Input Files")]
    NoInputFiles,

    #[error("Error: {file_name}: {error}")]
    FileRead {
        file_name: String,
        #[source]
        error: std::io::Error,
    },

    #[error(transparent)]
    Lexer(#[from] LexerError),

    #[error(transparent)]
    Syntax(#[from] SyntaxError),
}

pub type CompilerResult<T> = Result<T, CompilerError>;
