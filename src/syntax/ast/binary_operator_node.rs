use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub enum BinaryOperatorType {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub struct BinaryOperatorNode {
    op_type: BinaryOperatorType,
    left: Box<ASTNode>,
    right: Box<ASTNode>,
}

impl BinaryOperatorNode {
    pub fn new(
        op_type: BinaryOperatorType,
        left: ASTNode,
        right: ASTNode,
    ) -> Self {
        Self {
            op_type,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}
