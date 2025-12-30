use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct IfNode {
    condition_blocks: Vec<ConditionBlock>,
    else_body: Option<Vec<ASTNode>>,
}

impl IfNode {
    pub fn new(condition_blocks: Vec<ConditionBlock>, else_body: Option<Vec<ASTNode>>) -> Self {
        Self {
            condition_blocks,
            else_body
        }
    }
}

#[derive(Debug)]
pub struct ConditionBlock {
    condition: Box<ASTNode>,
    body: Vec<ASTNode>,
}

impl ConditionBlock {
    pub fn new(condition: ASTNode, body: Vec<ASTNode>) -> Self {
        Self {
            condition: Box::new(condition),
            body
        }
    }
}
