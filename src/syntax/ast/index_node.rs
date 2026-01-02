use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct IndexNode {
    operand: Box<ASTNode>,
    arg: Box<ASTNode>,
}

impl IndexNode {
    pub fn new(op_type: DerefType, operand: ASTNode, arg: ASTNode) -> Self {
        Self {
            operand: Box::new(operand),
            arg: Box::new(arg), 
        }
    }
}

#[derive(Debug)]
pub enum DerefType {
    Call,
    Index,
}
