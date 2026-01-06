use crate::ast::arena_ast::ASTNodeId;

#[derive(Debug)]
pub struct ForNode {
    item_identifier: String,
    iterator: ASTNodeId,
    body: Vec<ASTNodeId>,
}

impl ForNode {
    pub fn new(item_identifier: String, iterator: ASTNodeId, body: Vec<ASTNodeId>) -> Self {
        Self {
            item_identifier,
            iterator,
            body
        }
    }
}
