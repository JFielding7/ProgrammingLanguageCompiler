use crate::syntax::ast::access_node::AccessNode;
use crate::syntax::ast::ast_node::ASTNode::*;
use crate::syntax::ast::binary_operator_node::BinaryOperatorNode;
use crate::syntax::ast::for_node::ForNode;
use crate::syntax::ast::function_call_node::FunctionCallNode;
use crate::syntax::ast::index_node::IndexNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::ast::if_node::IfNode;
use crate::syntax::ast::unary_operator_node::UnaryOperatorNode;
use crate::syntax::ast::while_node::WhileNode;

#[derive(Debug)]
pub enum ASTNode {
    IntLiteral(String),
    StringLiteral(String),

    Identifier(String),

    UnaryOperator(UnaryOperatorNode),

    BinaryOperator(BinaryOperatorNode),

    FunctionDef(FunctionDefNode),

    FunctionCall(FunctionCallNode),

    Index(IndexNode),

    Access(AccessNode),
    
    If(IfNode),
    
    While(WhileNode),

    For(ForNode),
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

impl From<IndexNode> for ASTNode {
    fn from(node: IndexNode) -> Self {
        Index(node)
    }
}

impl From<AccessNode> for ASTNode {
    fn from(node: AccessNode) -> Self {
        Access(node)
    }
}

impl From<FunctionCallNode> for ASTNode {
    fn from(node: FunctionCallNode) -> Self {
        FunctionCall(node)
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

impl From<WhileNode> for ASTNode {
    fn from(node: WhileNode) -> Self {
        While(node)
    }
}

impl From<ForNode> for ASTNode {
    fn from(node: ForNode) -> Self {
        For(node)
    }
}
