use string_interner::DefaultSymbol;
use crate::ast::arena_ast::ASTNodeId;
use crate::types::type_annotation::TypeAnnotation;

#[derive(Debug)]
pub struct FunctionDefNode {
    name: DefaultSymbol,
    params: Vec<Parameter>,
    body: Vec<ASTNodeId>,
}

impl FunctionDefNode {
    pub fn new(
        name: DefaultSymbol,
        params: Vec<Parameter>,
        body: Vec<ASTNodeId>
    ) -> Self {
        Self {
            name,
            params,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    name: DefaultSymbol,
    type_annotation: TypeAnnotation,
}

impl Parameter {
    pub fn new(name: DefaultSymbol, type_annotation: TypeAnnotation) -> Self {
        Self { 
            name, 
            type_annotation 
        }
    }
}
