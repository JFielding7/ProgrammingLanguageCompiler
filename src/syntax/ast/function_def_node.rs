use crate::syntax::ast::ast_node::ASTNode;

#[derive(Debug)]
pub struct FunctionDefNode {
    name: String,
    params: Vec<Parameter>,
    body: Vec<ASTNode>,
}

impl FunctionDefNode {
    pub fn new(name: String, params: Vec<Parameter>, body: Vec<ASTNode>) -> Self {
        Self {
            name,
            params,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    name: String,
    data_type: String,
}

impl Parameter {
    pub fn new(name: String, data_type: String) -> Self {
        Self { name, data_type }
    }
}

