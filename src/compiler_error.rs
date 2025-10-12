use std::io;
use crate::lexer::{Token, TokenType};

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    NoInputFiles,

    InvalidToken(ErrorInfo),
    InvalidIndent(ErrorInfo),
    InvalidFunctionDef(ErrorInfo),
    InvalidIdentifier(ErrorInfo),

    ExpectTokenNotFound(Option<Token>, TokenType),

    #[error(transparent)]
    IOError(#[from] io::Error),
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub line_num: usize,
    pub start: usize,
    pub end: usize,
}

impl ErrorInfo {
    pub fn new(line_num: usize, start: usize, end: usize) -> Self {
        Self { line_num, start, end }
    }
}

fn format_error<'a>(error_info: &ErrorInfo, message: &str) -> String {
    format!("Error: Line {}: {message}", error_info.line_num)
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CompilerError::*;

        f.write_str(
            match self {
                NoInputFiles => "Error: No input files".to_string(),
                InvalidToken(info) => format_error(info, "Invalid Token"),
                InvalidIndent(info) => format_error(info, "Invalid Indent"),
                InvalidFunctionDef(info) => format_error(info, "Invalid Function Definition"),
                InvalidIdentifier(info) => format_error(info, "Identifier Expected"),
                e => e.to_string()
            }.as_str()
        )
    }
}

pub type Result<T> = std::result::Result<T, CompilerError>;
