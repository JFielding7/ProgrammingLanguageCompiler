use crate::source::source_span::SourceSpan;
use crate::syntax::ast::access_node::AccessNode;
use crate::syntax::ast::ast_node::ASTNodeType::*;
use crate::syntax::ast::binary_operator_node::BinaryOperatorNode;
use crate::syntax::ast::for_node::ForNode;
use crate::syntax::ast::function_call_node::FunctionCallNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::ast::if_node::IfNode;
use crate::syntax::ast::index_node::IndexNode;
use crate::syntax::ast::unary_operator_node::UnaryOperatorNode;
use crate::syntax::ast::while_node::WhileNode;

#[derive(Debug)]
pub struct ASTNode {
    pub node_type: ASTNodeType,
    pub span: SourceSpan,
}

impl ASTNode {
    pub fn new(node_type: ASTNodeType, span: SourceSpan) -> Self {
        Self { node_type, span }
    }
}

pub trait ASTNodeSpan {
    fn at(self, span: SourceSpan) -> ASTNode
    where Self:Sized, ASTNodeType: From<Self> {
        ASTNode::new(self.into(), span)
    }
}

#[derive(Debug)]
pub enum ASTNodeType {
    IntLiteral(String),
    StringLiteral(String),

    Identifier(String),

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

macro_rules! impl_to_ast_node_type {
    ($($node_type:ident => $variant:ident),*) => {
        $(
            impl From<$node_type> for ASTNodeType {
                fn from(node: $node_type) -> Self {
                    $variant(node)
                }
            }

            impl ASTNodeSpan for $node_type {}
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
