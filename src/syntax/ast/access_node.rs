use crate::syntax::ast::access_node::Member::Field;
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
    Field {
        name: String,
    },
    Method {
        name: String,
        args: Option<Box<ASTNode>>,
    },
}

impl Member {
    pub fn field(name: String) -> Self {
        Field {
            name,
        }
    }
    
    pub fn method_no_args(name: String) -> Self {
        Self::Method {
            name,
            args: None,
        }
    }

    pub fn method_with_args(name: String, args: ASTNode) -> Self {
        Self::Method {
            name,
            args: Some(Box::new(args)),
        }
    }
}
