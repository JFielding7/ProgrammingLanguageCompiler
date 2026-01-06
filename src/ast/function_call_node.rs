use crate::ast::arena_ast::ASTNodeId;

#[derive(Debug)]
pub struct FunctionCallNode {
    function: ASTNodeId,
    args: Option<ASTNodeId>,
}

impl FunctionCallNode {
    pub fn new(function: ASTNodeId, args: Option<ASTNodeId>) -> Self {
        Self {
            function,
            args,
        }
    }
}
