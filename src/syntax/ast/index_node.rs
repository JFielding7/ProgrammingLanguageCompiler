use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct IndexNode {
    operand: Box<ASTNode>,
    arg: Box<ASTNode>,
}

impl IndexNode {
    pub fn new(operand: ASTNode, arg: ASTNode) -> Self {
        Self {
            operand: Box::new(operand),
            arg: Box::new(arg), 
        }
    }
}
