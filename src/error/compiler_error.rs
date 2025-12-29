use std::{fmt, io};
use crate::error::error_info::ErrorInfo;
use crate::error::expected_token::ExpectedToken;
use crate::lexer::token::{Token, TokenType};

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    NoInputFiles,

    InvalidToken(ErrorInfo),
    InvalidIndent(ErrorInfo),

    ExpectedToken(ExpectedToken),

    UnmatchedParen(ErrorInfo),

    InvalidExpression(ErrorInfo),

    #[error(transparent)]
    IOError(#[from] io::Error),
}

fn format_error<'a>(error_info: &ErrorInfo, message: &str) -> String {
    format!("Error: Line {}: {message}", error_info.line_num)
}

fn format_expected_token_not_found(actual_token: &Option<Token>, expected_token_type: &TokenType) -> String {
    let message = match actual_token {
        None => format!("{expected_token_type} expected"),
        Some(token) => format!("expected {expected_token_type} but got {token}"),
    };

    format!("Error: {message}")
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CompilerError::*;

        f.write_str(
            match self {
                NoInputFiles => "Error: No input files".to_string(),
                InvalidToken(info) => format_error(info, "Invalid Token"),
                InvalidIndent(info) => format_error(info, "Invalid Indent"),
                UnmatchedParen(info) => format_error(info, "Unmatched Parentheses"),
                InvalidExpression(info) => format_error(info, "Invalid Expression"),
                ExpectedToken(expected_token) => "".to_string(), // TODO
                IOError(e) => e.to_string(),
            }.as_str()
        )
    }
}

pub type Result<T> = std::result::Result<T, CompilerError>;
