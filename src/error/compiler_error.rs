use CompilerError::*;
use crate::error::error_info::ErrorInfo;
use crate::error::expected_token::ExpectedTokenError;
use std::{fmt, io};

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    NoInputFiles,

    InvalidToken(ErrorInfo),
    
    InvalidIndent(ErrorInfo),

    ExpectedToken(ExpectedTokenError),

    UnmatchedParen(ErrorInfo),

    InvalidExpression(ErrorInfo),

    #[error(transparent)]
    IOError(#[from] io::Error),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoInputFiles => {
                write!(f, "Error: No input files")
            },

            InvalidToken(info) => {
                write!(f, "Invalid Token")
            },

            InvalidIndent(info) => {
                write!(f, "Invalid Indent")
            },
            
            UnmatchedParen(info) => {
                write!(f, "Unmatched Parentheses")
            },

            InvalidExpression(info) => {
                write!(f, "Invalid Expression")
            },

            ExpectedToken(expected_token) => {
                write!(f, "{}", expected_token)
            }

            IOError(e) => {
                write!(f, "{}", e)
            },
        }
    }
}

pub type Result<T> = std::result::Result<T, CompilerError>;
