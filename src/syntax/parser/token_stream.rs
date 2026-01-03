use crate::error_util::SourceLocation;
use crate::lexer::token::TokenType::Identifier;
use crate::lexer::token::{Token, TokenType};
use crate::syntax::error::expected_token::ExpectedTokenError;
use crate::syntax::error::SyntaxResult;
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
    
    pub fn prev_location(&self) -> SourceLocation {
        self.prev.location.clone()
    }

    pub fn next_token_of_type(&mut self, token_type: TokenType) -> SyntaxResult<&Token> {

        match self.next() {
            None => {
                Err(ExpectedTokenError::new(
                    None, token_type, self.prev_location()
                ).into())
            },

            Some(token) => {
                if *token == token_type {
                    Ok(token)
                } else {
                    let location = token.location.clone();
                    Err(ExpectedTokenError::new(Some(token.clone()), token_type, location).into())
                }
            },
        }
    }
    
    pub fn next_identifier(&mut self) -> SyntaxResult<String> {
        self.next_token_of_type(Identifier).map(|token| token.token_str.clone())
    }

    pub fn check_next_token(&mut self, token_type: TokenType) -> bool {
        self.peek().is_some_and(|&token| *token == token_type)
    }

    pub fn expect_next_token(&mut self, token_type: TokenType) -> SyntaxResult<bool> {

        match self.peek() {
            None => {
                Err(ExpectedTokenError::new(
                    None, token_type, self.prev_location()
                ).into())
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
