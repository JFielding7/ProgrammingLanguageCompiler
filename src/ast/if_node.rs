use crate::ast::arena_ast::ASTNodeId;

#[derive(Debug)]
pub struct IfNode {
    condition_blocks: Vec<ConditionBlock>,
    else_body: Option<Vec<ASTNodeId>>,
}

impl IfNode {
    pub fn new(condition_blocks: Vec<ConditionBlock>, else_body: Option<Vec<ASTNodeId>>) -> Self {
        Self {
            condition_blocks,
            else_body
        }
    }
}

#[derive(Debug)]
pub struct ConditionBlock {
    condition: ASTNodeId,
    body: Vec<ASTNodeId>,
}

impl ConditionBlock {
    pub fn new(condition: ASTNodeId, body: Vec<ASTNodeId>) -> Self {
        Self {
            condition,
            body
        }
    }
}
