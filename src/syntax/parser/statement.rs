use std::iter::Peekable;
use crate::error_util::SourceLocation;
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use crate::syntax::error::expected_token::ExpectedTokenError;
use crate::syntax::error::SyntaxResult;
use std::ops::Deref;
use std::vec::IntoIter;

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

pub struct StatementParser {
    iter: Peekable<IntoIter<Token>>,
    prev_token: Token,
}

impl StatementParser {
    pub fn from_suffix(mut statement: Statement, start: usize) -> Self {
        let prev_token = statement.tokens.remove(start - 1);

        Self {
            iter: statement.tokens.split_off(start).into_iter().peekable(),
            prev_token,
        }
    }

    pub fn next_token_of_type(&mut self, token_type: TokenType) -> SyntaxResult<Token> {

        match self.iter.next() {
            None => {
                Err(ExpectedTokenError::new(
                    None, token_type, self.prev_token.location.clone() // TODO: after token
                ).into())
            },

            Some(token) => {
                if token == token_type {
                    Ok(token)
                } else {
                    let location = token.location.clone();
                    Err(ExpectedTokenError::new(Some(token), token_type, location).into())
                }
            },
        }
    }

    pub fn cmp_next_token_type(&mut self, token_type: TokenType) -> SyntaxResult<bool> {
        let curr_token = self.iter.peek();

        match curr_token {
            None => {
                Err(ExpectedTokenError::new(
                    None, token_type, self.prev_token.location.clone() // TODO: after token
                ).into())
            }

            Some(token) => {
                Ok(*token == token_type)
            }
        }
    }
}
