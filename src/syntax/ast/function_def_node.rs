use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::parameter_node::ParameterNode;

#[derive(Debug)]
pub struct FunctionDefNode {
    name: String,
    params: Vec<ParameterNode>,
    body: Vec<ASTNode>,
}

impl FunctionDefNode {
    pub fn new(name: String, params: Vec<ParameterNode>) -> Self {
        Self {
            name,
            params,
            body: vec![]
        }
    }

    pub fn append_to_body(&mut self, node: ASTNode) {
        self.body.push(node);
    }
}

