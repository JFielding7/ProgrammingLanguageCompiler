use std::iter::Peekable;
use crate::error_util::SourceLocation;
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use crate::syntax::error::expected_token::ExpectedTokenError;
use crate::syntax::error::SyntaxResult;
use std::ops::Deref;
use std::vec::IntoIter;
use crate::syntax::parser::token_stream::TokenStream;

pub struct Statement {
    pub indent_size: usize,
    pub tokens: Vec<Token>,
}

impl Statement {
    pub const INDEX_AFTER_INDENT: usize = 1;
    
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            indent_size: Self::indent_size(&tokens),
            tokens,
        }
    }

    fn indent_size(tokens: &Vec<Token>) -> usize {
        let first_token = tokens
            .first()
            .expect("Statement must have at least one token");

        match first_token.token_type {
            Indent(size) => size,
            _ => unreachable!("Statement must start with Indent"),
        }
    }

    pub fn starts_with(&self, token_type: TokenType) -> bool {
        if self.len() > 1 {
            self[1] == token_type
        } else {
            unreachable!("Statement must not be blank")
        }
    }

    fn end_location(&self) -> SourceLocation {
        let last_token_location = &self.tokens
            .last()
            .expect("Statement must have at least one token")
            .location;

        SourceLocation::new(
            last_token_location.file_name.clone(),
            last_token_location.line_content.clone(),
            last_token_location.line_num,
            last_token_location.end,
            last_token_location.end + 1
        )
    }
}

impl Deref for Statement {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl IntoIterator for Statement {
    type Item = Token;
    type IntoIter = IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
