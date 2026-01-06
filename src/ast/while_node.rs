use crate::ast::arena_ast::ASTNodeId;

#[derive(Debug)]
pub struct WhileNode {
    condition: ASTNodeId,
    body: Vec<ASTNodeId>,
}

impl WhileNode {
    pub fn new(condition: ASTNodeId, body: Vec<ASTNodeId>) -> Self {
        Self {
            condition,
            body
        }
    }
}
