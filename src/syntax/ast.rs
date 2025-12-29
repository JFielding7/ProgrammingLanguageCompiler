pub mod function_def_node;
pub mod parameter_node;
pub mod binary_operator_node;
mod function_call_node;
pub mod ast_node;

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
