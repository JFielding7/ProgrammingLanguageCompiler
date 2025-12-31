use crate::syntax::ast::ast_node::ASTNode::*;
use crate::syntax::ast::binary_operator_node::BinaryOperatorNode;
use crate::syntax::ast::function_call_node::FunctionCallNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::ast::if_node::IfNode;
use crate::syntax::ast::unary_operator_node::UnaryOperatorNode;

#[derive(Debug)]
pub enum ASTNode {
    IntLiteral(String),
    StringLiteral(String),

    Identifier(String),

    UnaryOperator(UnaryOperatorNode),

    BinaryOperator(BinaryOperatorNode),

    FunctionDef(FunctionDefNode),

    FunctionCall(FunctionCallNode),
    
    If(IfNode),
}

impl From<UnaryOperatorNode> for ASTNode {
    fn from(node: UnaryOperatorNode) -> Self {
        UnaryOperator(node)
    }
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

impl From<IfNode> for ASTNode {
    fn from(node: IfNode) -> Self {
        If(node)
    }
}
