use crate::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct FunctionCallNode {
    function: Box<ASTNode>,
    params: Vec<ASTNode>,
}
