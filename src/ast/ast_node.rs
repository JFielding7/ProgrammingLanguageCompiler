use crate::source::source_span::SourceSpan;
use crate::ast::access_node::AccessNode;
use crate::ast::ast_node::ASTNodeType::*;
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::for_node::ForNode;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::IfNode;
use crate::ast::index_node::IndexNode;
use crate::ast::type_annotation::TypeAnnotation;
use crate::ast::unary_operator_node::UnaryOperatorNode;
use crate::ast::while_node::WhileNode;

#[derive(Debug)]
pub struct ASTNode {
    pub node_type: ASTNodeType,
    pub span: SourceSpan,
    pub type_annotation: Option<TypeAnnotation>,
}

impl ASTNode {
    pub fn new(node_type: ASTNodeType, span: SourceSpan) -> Self {
        Self { 
            node_type, 
            span,
            type_annotation: None,
        }
    }
}

#[derive(Debug)]
pub enum ASTNodeType {
    IntLiteral(String),
    StringLiteral(String),

    Variable(String),

    UnaryOperator(UnaryOperatorNode),

    BinaryOperator(BinaryOperatorNode),

    FunctionDef(FunctionDefNode),

    FunctionCall(FunctionCallNode),

    Index(IndexNode),

    Access(AccessNode),
    
    If(IfNode),
    
    While(WhileNode),

    For(ForNode),
}

pub trait SpannableASTNode {
    fn at(self, span: SourceSpan) -> ASTNode
    where Self: Sized, ASTNodeType: From<Self> {
        ASTNode::new(self.into(), span)
    }
}

macro_rules! impl_to_ast_node_type {
    ($($node_type:ident => $variant:ident),*) => {
        $(
            impl From<$node_type> for ASTNodeType {
                fn from(node: $node_type) -> Self {
                    $variant(node)
                }
            }

            impl SpannableASTNode for $node_type {}
        )*
    };
}

impl_to_ast_node_type! {
    UnaryOperatorNode => UnaryOperator,
    BinaryOperatorNode => BinaryOperator,
    IndexNode => Index,
    AccessNode => Access,
    FunctionCallNode => FunctionCall,
    FunctionDefNode => FunctionDef,
    IfNode => If,
    WhileNode => While,
    ForNode => For
}
