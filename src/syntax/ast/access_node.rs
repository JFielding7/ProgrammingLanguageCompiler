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
    Method(String, Option<Box<ASTNode>>),
}

impl Member {
    pub fn method_no_args(method_name: String) -> Self {
        Self::Method(method_name, None)
    }

    pub fn method_with_args(method_name: String, args: ASTNode) -> Self {
        Self::Method(method_name, Some(Box::new(args)))
    }
}
