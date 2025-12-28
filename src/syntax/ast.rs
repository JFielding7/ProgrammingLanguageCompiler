pub mod function_def_node;
pub mod parameter_node;
pub mod binary_operator_node;
mod function_call_node;
pub mod ast_node;

use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;


pub struct AST {
    file_name: String,
    functions: Vec<FunctionDefNode>,
    top_level_code: Vec<ASTNode>,
}

impl AST {
    pub fn new(file_name: String, functions: Vec<FunctionDefNode>, top_level_code: Vec<ASTNode>) -> Self {
        Self {
            file_name,
            functions,
            top_level_code,
        }
    }
}
