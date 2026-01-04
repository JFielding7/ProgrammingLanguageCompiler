use std::iter::Peekable;
use crate::source::source_span::SourceSpan;
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

    pub fn start_token_type(&self) -> &TokenType {
        if self.len() > Self::INDEX_AFTER_INDENT {
            &self[Self::INDEX_AFTER_INDENT].token_type
        } else {
            unreachable!("Statement must not be blank")
        }
    }

    pub fn starts_with(&self, token_type: TokenType) -> bool {
        self.start_token_type() == &token_type
    }

    pub fn suffix(&self, start: usize) -> &[Token] {
        &self.tokens[start..]
    }

    pub fn suffix_token_stream(&self, start: usize) -> TokenStream<'_> {
        TokenStream::new(&self.tokens[start..])
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
