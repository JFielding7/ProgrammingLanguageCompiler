pub mod function_def_node;
pub mod binary_operator_node;
pub mod index_node;
pub mod ast_node;
pub mod unary_operator_node;
pub mod if_node;
pub mod access_node;
pub mod function_call_node;
pub mod while_node;
pub mod for_node;
pub mod type_annotation;
pub mod ast_arena;

use crate::ast::ast_arena::ASTNodeId;


#[derive(Debug)]
pub struct AST {
    functions: Vec<ASTNodeId>,
    top_level_code: Vec<ASTNodeId>,
}

impl AST {
    pub fn new(functions: Vec<ASTNodeId>, top_level_code: Vec<ASTNodeId>) -> Self {
        Self {
            functions,
            top_level_code,
        }
    }
}
