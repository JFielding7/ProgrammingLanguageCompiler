use crate::ast::arena_ast::ASTNodeId;

#[derive(Debug)]
pub struct IndexNode {
    operand: ASTNodeId,
    arg: ASTNodeId,
}

impl IndexNode {
    pub fn new(operand: ASTNodeId, arg: ASTNodeId) -> Self {
        Self {
            operand,
            arg
        }
    }
}
