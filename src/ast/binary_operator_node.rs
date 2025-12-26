use crate::ast::ast_tree::ASTNode;
use crate::ast::binary_operator_node::BinaryOperatorType::*;
use crate::lexer::token::TokenType::*;
use crate::lexer::token::Token;

#[derive(Debug)]
pub struct BinaryOperatorNode {
    op: BinaryOperatorType,
    left: Box<ASTNode>,
    right: Box<ASTNode>,
}

impl BinaryOperatorNode {
    pub fn new(
        op_token: &Token,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    ) -> Self {
        Self {
            op: BinaryOperatorType::new(op_token),
            left,
            right
        }
    }
}

#[derive(Debug)]
enum BinaryOperatorType {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOperatorType {
    fn new(op_token: &Token) -> Self {
        match op_token.token_type {
            Plus => Add,
            Minus => Sub,
            _ => Mul, // TODO
        }
    }
}