use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub enum UnaryOperatorType {
    Neg,
    Not,
}

#[derive(Debug)]
pub struct UnaryOperatorNode {
    op_type: UnaryOperatorType,
    operand: Box<ASTNode>,
}

impl UnaryOperatorNode {
    pub fn new(
        op_type: UnaryOperatorType,
        operand: ASTNode,
    ) -> Self {
        Self {
            op_type,
            operand: Box::new(operand)
        }
    }
}
