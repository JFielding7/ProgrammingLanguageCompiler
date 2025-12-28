use crate::syntax::ast::ast_node::ASTNode::{BinaryOperator, FunctionDef};
use crate::syntax::ast::binary_operator_node::BinaryOperatorNode;
use crate::syntax::ast::function_call_node::FunctionCallNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;

#[derive(Debug)]
pub enum ASTNode {
    IntLiteral(String),
    StringLiteral(String),

    Identifier(String),

    BinaryOperator(BinaryOperatorNode),

    FunctionDef(FunctionDefNode),

    FunctionCall(FunctionCallNode),
}

impl From<BinaryOperatorNode> for ASTNode {
    fn from(node: BinaryOperatorNode) -> Self {
        BinaryOperator(node)
    }
}

impl From<FunctionDefNode> for ASTNode {
    fn from(node: FunctionDefNode) -> Self {
        FunctionDef(node)
    }
}
