use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct ForNode {
    item_identifier: String,
    iterator: Box<ASTNode>,
    body: Vec<ASTNode>,
}

impl ForNode {
    pub fn new(item_identifier: String, iterator: ASTNode, body: Vec<ASTNode>) -> Self {
        Self {
            item_identifier,
            iterator: Box::new(iterator),
            body
        }
    }
}
