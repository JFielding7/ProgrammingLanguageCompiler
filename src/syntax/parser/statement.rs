use std::ops::Index;
use crate::lexer::token::TokenType::{Fn, Indent};
use crate::lexer::token::{Token, TokenType};
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::parser::expression::ExpressionParser;
use crate::syntax::parser::function::parse_function_def;
use crate::syntax::parser::source_statements::SourceStatementsIter;
use crate::error_util::ErrorLocation;
use crate::syntax::error::expected_token::ExpectedTokenError;
use crate::syntax::error::SyntaxResult;

pub struct Statement {
    pub indent_size: usize,
    pub tokens: Vec<Token>,
}

impl Statement {
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

    pub fn to_ast_node(self, next_statements: &mut SourceStatementsIter) -> SyntaxResult<ASTNode> {

        match self[1].token_type {
            Fn => Ok(parse_function_def(self, next_statements)?.into()),
            _ => ExpressionParser::parse(&self),
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    fn end_error_location(&self) -> ErrorLocation {
        let last_token_error_info = &self.tokens
            .last()
            .expect("Statement must have at least one token")
            .error_location;

        ErrorLocation::new(
            last_token_error_info.file_name.clone(),
            last_token_error_info.line_content.clone(),
            last_token_error_info.line_num,
            last_token_error_info.end,
            last_token_error_info.end + 1
        )
    }
}

impl Index<usize> for Statement {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
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
                None, token_type, statement.end_error_location()
            ).into())
        }

        let token = statement[self.curr_token_index].clone();
        self.curr_token_index += 1;

        if token == token_type {
            Ok(token)
        } else {
            let error_info = token.error_location.clone();
            Err(ExpectedTokenError::new(Some(token), token_type, error_info).into())
        }
    }

    pub fn cmp_next_token_type(&self, token_type: TokenType) -> SyntaxResult<bool> {
        let curr_token_index = self.curr_token_index;
        let statement = self.statement;

        if curr_token_index >= statement.len() {
            Err(ExpectedTokenError::new(
                None, token_type, statement.end_error_location()
            ).into())
        } else {
            Ok(statement[curr_token_index] == token_type)
        }
    }

    pub fn skip(&mut self, n: usize) {
        self.curr_token_index += n;
    }

    pub fn is_next_token_of_type(&self, token_type: TokenType) -> bool {
        self.statement[self.curr_token_index] == token_type
    }
}
