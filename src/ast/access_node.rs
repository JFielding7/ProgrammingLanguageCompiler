use crate::ast::access_node::Member::Field;
use crate::ast::ast_arena::ASTNodeId;

#[derive(Debug)]
pub struct AccessNode {
    receiver: ASTNodeId,
    member: Member,
}

impl AccessNode {
    pub fn new(receiver: ASTNodeId, member: Member) -> Self {
        Self {
            receiver,
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
        args: Option<ASTNodeId>,
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

    pub fn method_with_args(name: String, args: ASTNodeId) -> Self {
        Self::Method {
            name,
            args: Some(args),
        }
    }
}
