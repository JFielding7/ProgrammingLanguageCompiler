use crate::error::compiler_error::Result;
use crate::lexer::token::Token;
use crate::lexer::token::TokenType::Fn;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::parser::expression::ExpressionParser;
use crate::syntax::parser::function::parse_function_def;
use std::iter::Peekable;
use std::ops::Index;
use std::vec::IntoIter;

type StatementIter = Peekable<IntoIter<Statement>>;

pub struct Statement {
    pub indent_size: usize,
    pub tokens: Vec<Token>,
}

impl Statement {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            indent_size: Self::indent_size(&tokens),
            tokens
        }
    }

    fn indent_size(tokens: &Vec<Token>) -> usize {
        tokens.first()
            .expect("Statement must have at least one token")
            .indent_size()
            .expect("First token must be indent")
    }

    pub fn to_ast_node(self, next_statements: &mut StatementIter) -> Result<ASTNode> {

        match self[1].token_type {
            Fn => Ok(parse_function_def(self, next_statements)?.into()),
            _ => ExpressionParser::parse(&self),
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

impl IntoIterator for Statement {
    type Item = Token;
    type IntoIter = IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl Index<usize> for Statement {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}

pub trait StatementIterMethods {
    fn ast_child_nodes(&mut self, parent_indent_size: usize) -> Result<Vec<ASTNode>>;
}

impl StatementIterMethods for StatementIter {
    fn ast_child_nodes(&mut self, parent_indent_size: usize) -> Result<Vec<ASTNode>> {
        let mut child_nodes = Vec::new();

        while let Some(child) = self.peek() {

            if child.indent_size <= parent_indent_size {
                break;
            }

            let statement = self.next().unwrap();
            child_nodes.push(statement.to_ast_node(self)?)
        }

        Ok(child_nodes)
    }
}
