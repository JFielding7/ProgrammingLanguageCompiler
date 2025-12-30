use crate::error_util::SourceLocation;
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use crate::syntax::error::expected_token::ExpectedTokenError;
use crate::syntax::error::SyntaxResult;
use std::ops::Deref;

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

pub struct StatementParser<'a> {
    statement: &'a Statement,
    curr_token_index: usize,
}

impl<'a> StatementParser<'a> {
    pub fn new(statement: &'a Statement) -> Self {
        Self {
            statement,
            curr_token_index: 0,
        }
    }

    pub fn next_token_of_type(&mut self, token_type: TokenType) -> SyntaxResult<Token> {
        let statement = self.statement;

        if self.curr_token_index >= statement.len() {
            return Err(ExpectedTokenError::new(
                None, token_type, statement.end_location()
            ).into())
        }

        let token = statement[self.curr_token_index].clone();
        self.curr_token_index += 1;

        if token == token_type {
            Ok(token)
        } else {
            let location = token.location.clone();
            Err(ExpectedTokenError::new(Some(token), token_type, location).into())
        }
    }

    pub fn cmp_next_token_type(&self, token_type: TokenType) -> SyntaxResult<bool> {
        let curr_token_index = self.curr_token_index;
        let statement = self.statement;

        if curr_token_index >= statement.len() {
            Err(ExpectedTokenError::new(
                None, token_type, statement.end_location()
            ).into())
        } else {
            Ok(statement[curr_token_index] == token_type)
        }
    }

    pub fn skip(&mut self, n: usize) {
        self.curr_token_index += n;
    }
}
