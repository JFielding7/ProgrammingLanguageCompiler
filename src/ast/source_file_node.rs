use crate::ast::function_def_node::FunctionDefNode;

#[derive(Debug)]
pub struct SourceFileNode {
    name: String,
    functions: Vec<FunctionDefNode>
}

impl SourceFileNode {
    pub fn new(name: String, functions: Vec<FunctionDefNode>) -> Self {
        SourceFileNode {
            name,
            functions
        }
    }
}
