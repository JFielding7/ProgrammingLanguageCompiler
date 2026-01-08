use crate::ast::arena_ast::ASTNodeId;
use crate::operators::binary_operators::BinaryOperatorType;

#[derive(Debug)]
pub struct BinaryOperatorNode {
    pub op_type: BinaryOperatorType,
    pub left: ASTNodeId,
    pub right: ASTNodeId,
}

impl BinaryOperatorNode {
    pub fn new(
        op_type: BinaryOperatorType,
        left: ASTNodeId,
        right: ASTNodeId,
    ) -> Self {
        Self {
            op_type,
            left,
            right,
        }
    }
}
