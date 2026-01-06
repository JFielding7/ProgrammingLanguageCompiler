use crate::ast::ast_node::ASTNodeType::FunctionDef;
use crate::ast::ast_node::ASTNode;
use crate::ast::type_annotation::TypeAnnotation;

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

    pub fn annotate(&mut self, id: ASTNodeId, type_annotation: Option<TypeAnnotation>) -> ASTNodeId {
        self.node_arena[id.0].type_annotation = type_annotation;
        id
    }

    pub fn lookup(&self, id: ASTNodeId) -> &ASTNode {
        &self.node_arena[id.0]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ASTNodeId(usize);
