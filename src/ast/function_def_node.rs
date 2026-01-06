use crate::ast::arena_ast::ASTNodeId;
use crate::ast::type_annotation::TypeAnnotation;

#[derive(Debug)]
pub struct FunctionDefNode {
    name: String,
    params: Vec<Parameter>,
    body: Vec<ASTNodeId>,
}

impl FunctionDefNode {
    pub fn new(name: String, params: Vec<Parameter>, body: Vec<ASTNodeId>) -> Self {
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
    type_annotation: TypeAnnotation,
}

impl Parameter {
    pub fn new(name: String, type_annotation: TypeAnnotation) -> Self {
        Self { 
            name, 
            type_annotation 
        }
    }
}

