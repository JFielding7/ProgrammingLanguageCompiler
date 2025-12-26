use crate::statement::SourceFileStatements;
use crate::ast::ast_node::ASTNode::*;
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::parameter_node::ParameterNode;
use crate::ast::source_file_node::SourceFileNode;
use crate::error::compiler_error::Result;
use crate::error::compiler_error::CompilerError::InvalidExpression;
use crate::token::{Token};
use crate::token::TokenType::*;

#[derive(Debug)]
pub enum ASTNode {
    IntLiteral(String),
    StringLiteral(String),

    Identifier(String),

    Parameter(ParameterNode),

    BinaryOperator(BinaryOperatorNode),

    FunctionDefinition(FunctionDefNode),

    FunctionCall(FunctionCallNode),

    SourceFile(SourceFileNode)
}

impl ASTNode {
    pub fn int_iteral(literal: String) -> ASTNode {
        ASTNode::IntLiteral(literal)
    }

    pub fn string_iteral(literal: String) -> ASTNode {
        ASTNode::StringLiteral(literal)
    }

    pub fn identifier(identifier: String) -> ASTNode {
        ASTNode::Identifier(identifier)
    }
    
    pub fn binary_operator(
        op_token: Token,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    ) -> ASTNode {
        BinaryOperator(BinaryOperatorNode::new(
            &op_token,
            left,
            right
        ))
    }
}

fn get_ast_node(statement: Vec<Token>) -> Result<ASTNode> {
    let mut tokens = statement.into_iter();

    // let indent_size = tokens.next().indent_size()?;
    let token = tokens.next().unwrap();

    match token.token_type {
        Fn => Ok(FunctionDefNode::new(tokens)?),
        _ => Err(InvalidExpression(token.error_info))
    }
}

pub fn build_ast(statements: SourceFileStatements) -> Result<ASTNode> {
    for statement in statements.statements {
        println!("{statement:?}");
        println!("{:?}", get_ast_node(statement));
    }

    Ok(SourceFile(SourceFileNode::new(statements.file_name, vec![])))
}
