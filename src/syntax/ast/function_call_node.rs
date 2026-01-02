use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct FunctionCallNode {
    function: Box<ASTNode>,
    args: Option<Box<ASTNode>>,
}

impl FunctionCallNode {
    pub fn new(function: ASTNode, args: Option<ASTNode>) -> Self {
        Self {
            function: Box::new(function),
            args: args.map(Box::new),
        }
    }
}
