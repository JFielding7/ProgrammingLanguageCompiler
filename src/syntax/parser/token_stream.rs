use crate::source::source_span::SourceSpan;
use crate::lexer::token::TokenType::Identifier;
use crate::lexer::token::{Token, TokenType};
use crate::error::spanned_error::WithSpan;
use crate::syntax::error::{SyntaxErrorType, SyntaxResult};
use std::iter::Peekable;
use std::slice::Iter;

pub struct TokenStream<'a> {
    iter: Peekable<Iter<'a, Token>>,
    prev: &'a Token,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            iter: tokens.iter().peekable(),
            prev: &tokens[0]
        }
    }

    pub fn peek(&mut self) -> Option<&&'a Token> {
        self.iter.peek()
    }

    pub fn empty(&mut self) -> bool{
        self.iter.peek().is_none()
    }
    
    pub fn prev_span(&self) -> SourceSpan {
        self.prev.span.clone()
    }

    pub fn expect_next_token(&mut self, token_type: TokenType) -> SyntaxResult<&Token> {

        match self.next() {
            None => {
                Err(SyntaxErrorType::expected_token(None, token_type).at(self.prev_span()))
            },

            Some(token) => {
                if *token == token_type {
                    Ok(token)
                } else {
                    let span = token.span.clone();
                    Err(SyntaxErrorType::expected_token(Some(token.clone()), token_type).at(span))
                }
            },
        }
    }
    
    pub fn expect_next_identifier(&mut self) -> SyntaxResult<String> {
        self.expect_next_token(Identifier).map(|token| token.token_str.clone())
    }

    pub fn next_matches(&mut self, token_type: TokenType) -> bool {
        self.peek().is_some_and(|&token| *token == token_type)
    }

    pub fn required_next_matches(&mut self, token_type: TokenType) -> SyntaxResult<bool> {

        match self.peek() {
            None => {
                Err(SyntaxErrorType::expected_token(None, token_type).at(self.prev_span()))
            }

            Some(&token) => {
                Ok(*token == token_type)
            }
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.iter.next();

        if let Some(t) = token {
            self.prev = t;
        }

        token
    }
}
