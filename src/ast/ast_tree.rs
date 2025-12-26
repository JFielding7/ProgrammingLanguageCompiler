use std::iter::Peekable;
use crate::ast::ast_tree::ASTNode::*;
use crate::ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::parameter_node::ParameterNode;
use crate::ast::source_file_node::SourceFileNode;
use crate::error::compiler_error::Result;
use crate::error::compiler_error::CompilerError::InvalidExpression;
use crate::lexer::lexer::TokenizedSource;
use crate::parser::statement::{ParsedSource, Statement};
use crate::lexer::token::{Token};
use crate::lexer::token::TokenType::*;
use std::vec::IntoIter;


pub type TokenIter = Peekable<IntoIter<Token>>;

#[derive(Debug)]
pub enum ASTNode {
    IntLiteral(String),
    StringLiteral(String),

    Identifier(String),

    Parameter(ParameterNode),

    BinaryOperator(BinaryOperatorNode),

    FunctionDef(FunctionDefNode),

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

fn get_ast_node(statement: Statement) -> Result<ASTNode> {
    let mut tokens = statement.into_iter().peekable();

    let indent_size = tokens.next();
    let token = tokens.next().unwrap();

    match token.token_type {
        Fn => Ok(FunctionDef(FunctionDefNode::parse(tokens)?)),
        _ => Err(InvalidExpression(token.error_info))
    }
}

pub fn build_ast(tokens: TokenizedSource) -> Result<ASTNode> {

    let file_name = tokens.file_name.to_owned();
    let statements = ParsedSource::new(tokens);

    for statement in statements {
        let node = get_ast_node(statement)?;
        println!("{node:?}");
    }

    Ok(SourceFile(SourceFileNode::new(file_name, vec![])))
}
