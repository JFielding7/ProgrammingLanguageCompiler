use std::{fmt, io};
use crate::lexer::token::{Token, TokenType};

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    NoInputFiles,

    InvalidToken(ErrorInfo),
    InvalidIndent(ErrorInfo),
    InvalidIdentifier(ErrorInfo),

    ExpectTokenNotFound(Option<Token>, TokenType),

    UnmatchedParen(ErrorInfo),

    InvalidExpression(ErrorInfo),

    #[error(transparent)]
    IOError(#[from] io::Error),
}

#[derive(Debug, Copy, Clone)]
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

// fn format_expected_token_not_found(token: &Option<Token>, token_type: &TokenType) {
//     let error_info = match token {
//         Some(t) => t.error_info,
//         None => return format_error()
//     }
//     format!("Error: Line {}: {message}", error_info.line_num)
// }

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CompilerError::*;

        f.write_str(
            match self {
                NoInputFiles => "Error: No input files".to_string(),
                InvalidToken(info) => format_error(info, "Invalid Token"),
                InvalidIndent(info) => format_error(info, "Invalid Indent"),
                InvalidIdentifier(info) => format_error(info, "Identifier Expected"),
                ExpectTokenNotFound(token, token_type) => {
                    // todo: fix
                    "Expected token not found".to_string()
                },
                UnmatchedParen(info) => format_error(info, "Unmatched Parentheses"),
                InvalidExpression(info) => format_error(info, "Invalid Expression"),
                IOError(e) => e.to_string(),
            }.as_str()
        )
    }
}

pub type Result<T> = std::result::Result<T, CompilerError>;
