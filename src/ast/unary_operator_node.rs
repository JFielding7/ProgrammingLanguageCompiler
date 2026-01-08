use crate::ast::arena_ast::ASTNodeId;
use crate::operators::unary_operators::UnaryOperatorType;

#[derive(Debug)]
pub struct UnaryOperatorNode {
    pub op_type: UnaryOperatorType,
    pub operand: ASTNodeId,
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
