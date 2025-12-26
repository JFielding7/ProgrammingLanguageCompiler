use crate::ast::ast_tree::ASTNode;

#[derive(Debug)]
pub struct FunctionCallNode {
    function: Box<ASTNode>,
    params: Vec<ASTNode>,
}
