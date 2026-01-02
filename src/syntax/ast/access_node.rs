use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct AccessNode {
    receiver: Box<ASTNode>,
    member: Member,
}

impl AccessNode {
    pub fn new(receiver: ASTNode, member: Member) -> Self {
        Self {
            receiver: Box::new(receiver),
            member,
        }
    }
}

#[derive(Debug)]
pub enum Member {
    Field(String),
    Method(String, Box<ASTNode>),
}
