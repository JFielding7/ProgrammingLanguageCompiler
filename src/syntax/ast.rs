pub mod function_def_node;
pub mod parameter_node;
pub mod binary_operator_node;
pub mod index_node;
pub mod ast_node;
pub mod unary_operator_node;
pub mod if_node;
pub mod access_node;
pub mod function_call_node;

use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;


#[derive(Debug)]
pub struct AST {
    functions: Vec<FunctionDefNode>,
    top_level_code: Vec<ASTNode>,
}

impl AST {
    pub fn new(functions: Vec<FunctionDefNode>, top_level_code: Vec<ASTNode>) -> Self {
        Self {
            functions,
            top_level_code,
        }
    }
}
