use crate::error::compiler_error::Result;
use crate::error::expected_token::ExpectedToken;
use crate::lexer::token::TokenType::Fn;
use crate::lexer::token::{Token, TokenType};
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::parser::expression::ExpressionParser;
use crate::syntax::parser::function::parse_function_def;
use crate::syntax::parser::source_statements::SourceStatementsIter;
use std::ops::Index;

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
        tokens.first()
            .expect("Statement must have at least one token")
            .indent_size()
            .expect("First token must be indent")
    }

    pub fn to_ast_node(self, next_statements: &mut SourceStatementsIter) -> Result<ASTNode> {

        match self[1].token_type {
            Fn => Ok(parse_function_def(self, next_statements)?.into()),
            _ => ExpressionParser::parse(&self),
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
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
    
    pub fn next_token_of_type(&mut self, token_type: TokenType) -> Result<Token> {
        let statement = self.statement;

        if self.curr_token_index >= statement.len() {
            return ExpectedToken::new(None, token_type).into()
        }

        let token = statement[self.curr_token_index].clone();

        if token == token_type {
            Ok(token)
        } else {
            ExpectedToken::new(Some(token), token_type).into()
        }
    }
    
    pub fn peek(&self) -> Option<&Token> {
        let curr_token_index = self.curr_token_index;
        let statement = self.statement;
        
        if curr_token_index >= statement.len() {
            None
        } else {
            Some(&statement[curr_token_index])
        }
    }
    
    pub fn is_next_token_of_type(&self, token_type: TokenType) -> bool {
        self.statement[self.curr_token_index] == token_type
    }
}
