use std::cmp::PartialEq;
use logos::Logos;
use std::fmt;
use std::mem::discriminant;
use TokenType::*;
use crate::error::compiler_error::Result;
use crate::error::error_info::ErrorInfo;
use crate::error::expected_token::ExpectedToken;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_str: String,
    pub error_info: ErrorInfo,
}

#[derive(Logos, Debug, Clone)]
pub enum TokenType {
    #[token("fn")]
    Fn,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("=")]
    Assign,

    #[regex(r"[0-9]+")]
    IntLiteral,
    #[regex(r#""[^"]*""#)]
    StringLiteral,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token(",")]
    Comma,

    #[regex(r"[ \t\f\v]+", logos::skip)]
    Whitespace,
    #[regex(r"//.*", logos::skip)]
    Comment,

    Indent(usize),
}

impl Token {
    pub fn new(token_type: TokenType, token_str: String, error_info: ErrorInfo) -> Self {
        Self { token_type, token_str, error_info }
    }

    pub fn is_legal_statement_boundary(&self) -> bool {
        match self.token_type {
            Plus |
            Minus |
            Assign |
            OpenParen |
            Comma => false,
            _ => true,
        }
    }

    pub fn indent_size(&self) -> Result<usize> {
        match self.token_type {
            Indent(size) => Ok(size),
            _ => panic!("Expected indent token")
        }
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        discriminant(&self.token_type) == discriminant(other)
    }
}

pub trait TokenOpt {
    fn assert_type(self, token_type: TokenType) -> Result<Token>;
}

impl TokenOpt for Option<Token> {
    fn assert_type(self, token_type: TokenType) -> Result<Token> {
        match self {
            None => ExpectedToken::new(None, token_type).into(),
            Some(token) => {
                if token == token_type {
                    Ok(token)
                } else {
                    ExpectedToken::new(Some(token), token_type).into()
                }
            }
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token_str.as_str())
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            OpenParen => "(",
            CloseParen => ")",
            Comma => ",",
            Indent(_) => "Indent",
            _ => ""
        })
    }
}
