use std::iter::Map;
use std::ops::Range;
use crate::ast::ast_node::ASTNodeType::FunctionDef;
use crate::ast::ast_node::ASTNode;
use crate::types::type_annotation::TypeAnnotation;


#[derive(Debug)]
pub struct AST {
    functions: Vec<ASTNodeId>,
    top_level_code: Vec<ASTNodeId>,
    node_arena: Vec<ASTNode>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            functions: vec![],
            top_level_code: vec![],
            node_arena: vec![]
        }
    }

    pub fn add_node(&mut self, node: ASTNode) -> ASTNodeId {
        let id = self.node_arena.len();
        self.node_arena.push(node);
        ASTNodeId(id)
    }

    pub fn add_top_level_node(&mut self, node_id: ASTNodeId) {
        match self.lookup(node_id).node_type {
            FunctionDef(_) => self.functions.push(node_id),
            _ => self.top_level_code.push(node_id)
        }
    }

    pub fn lookup(&self, id: ASTNodeId) -> &ASTNode {
        &self.node_arena[id.0]
    }

    pub fn lookup_mut(&mut self, id: ASTNodeId) -> &mut ASTNode {
        &mut self.node_arena[id.0]
    }
    
    pub fn ast_node_id_iter(&self) -> NodeIdIter {
        (0..self.node_arena.len()).map(|id| ASTNodeId(id))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ASTNodeId(pub usize);
pub type NodeIdIter = Map<Range<usize>, fn(usize) -> ASTNodeId>;
