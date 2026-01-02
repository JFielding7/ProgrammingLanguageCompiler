use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct WhileNode {
    condition: Box<ASTNode>,
    body: Vec<ASTNode>,
}

impl WhileNode {
    pub fn new(condition: ASTNode, body: Vec<ASTNode>) -> Self {
        Self {
            condition: Box::new(condition),
            body
        }
    }
}
