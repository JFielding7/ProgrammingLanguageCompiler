use crate::ast::arena_ast::ASTNodeId;

#[derive(Debug)]
pub struct UnaryOperatorNode {
    op_type: UnaryOperatorType,
    operand: ASTNodeId,
}

impl UnaryOperatorNode {
    pub fn new(
        op_type: UnaryOperatorType,
        operand: ASTNodeId,
    ) -> Self {
        Self {
            op_type,
            operand
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperatorType {
    Neg,
    Not,
    BitNot,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}
