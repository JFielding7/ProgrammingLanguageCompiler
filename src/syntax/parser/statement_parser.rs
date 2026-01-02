use crate::lexer::token::{Token, TokenType};
use crate::syntax::error::expected_token::ExpectedTokenError;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::statement::Statement;
use crate::syntax::parser::token_stream::TokenStream;

pub struct StatementParser<'a> {
    token_stream: TokenStream<'a>,
}

impl<'a> StatementParser<'a> {
    pub fn from_suffix(statement: &'a Statement, start: usize) -> Self {
        Self {
            token_stream: TokenStream::from_statement_suffix(statement, start),
        }
    }

    pub fn next_token_of_type(&mut self, token_type: TokenType) -> SyntaxResult<&Token> {

        match self.token_stream.next() {
            None => {
                Err(ExpectedTokenError::new(
                    None, token_type, self.token_stream.prev_location()
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

    pub fn cmp_next_token_type(&mut self, token_type: TokenType) -> SyntaxResult<bool> {
        let curr_token = self.token_stream.peek();

        match curr_token {
            None => {
                Err(ExpectedTokenError::new(
                    None, token_type, self.token_stream.prev_location()
                ).into())
            }

            Some(&token) => {
                Ok(*token == token_type)
            }
        }
    }
}
