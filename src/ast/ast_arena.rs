use crate::ast::ast_node::{ASTNode, ASTNodeType};
use crate::ast::type_annotation::TypeAnnotation;
use crate::source::source_span::SourceSpan;

pub struct ASTArena {
    nodes: Vec<ASTNode>,
}

impl ASTArena {
    pub fn new() -> Self {
        ASTArena { nodes: vec![] }
    }

    pub fn add_node(&mut self, node: ASTNode) -> ASTNodeId {
        let id = self.nodes.len();
        self.nodes.push(node);
        ASTNodeId(id)
    }

    pub fn add_with_span<T>(&mut self, node: T, span: SourceSpan) -> ASTNodeId
    where T:Sized, ASTNodeType: From<T> {
        let id = self.nodes.len();
        self.nodes.push(ASTNode::new(node.into(), span));
        ASTNodeId(id)
    }

    pub fn annotate(&mut self, id: ASTNodeId, type_annotation: Option<TypeAnnotation>) -> ASTNodeId {
        self.nodes[id.0].type_annotation = type_annotation;
        id
    }
    
    pub fn lookup(&self, id: ASTNodeId) -> &ASTNode {
        &self.nodes[id.0]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ASTNodeId(usize);
