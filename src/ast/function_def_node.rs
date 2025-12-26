use crate::ast::ast_tree::{ASTNode, TokenIter};
use crate::ast::parameter_node::ParameterNode;
use crate::error::compiler_error::Result;
use crate::lexer::token::TokenType::{CloseParen, Comma, Identifier, OpenParen};
use crate::lexer::token::{Token, TokenOpt};
use std::vec::IntoIter;

#[derive(Debug)]
pub struct FunctionDefNode {
    name: String,
    params: Vec<ParameterNode>,
    body: Vec<ASTNode>,
}

impl FunctionDefNode {
    pub fn new(name: String, params: Vec<ParameterNode>) -> Self {
        Self { name, params, body: vec![] }
    }

    pub fn parse(mut tokens: TokenIter) -> Result<Self> {

        let name = parse_function_name(&mut tokens)?;
        let params = parse_parameters(&mut tokens)?;

        // TODO: parse body

        Ok(Self::new(name, params))
    }
}

fn parse_function_name(tokens: &mut TokenIter) -> Result<String> {
    tokens.next().assert_type(Identifier).map(|token| token.token_str)
}

fn parse_parameters(tokens: &mut TokenIter) -> Result<Vec<ParameterNode>> {
    tokens.next().assert_type(OpenParen)?;

    let mut params = Vec::new();

    if let Some(token) = tokens.peek() {
        if *token == CloseParen {
            return Ok(params);
        }
    }

    params.push(parse_parameter(tokens)?);

    while tokens.next_if(|token| *token == Comma).is_some() {
        params.push(parse_parameter(tokens)?);
    }

    tokens.next().assert_type(CloseParen)?;
    Ok(params)
}

fn parse_parameter(tokens: &mut TokenIter) -> Result<ParameterNode> {
    let param_type = tokens.next().assert_type(Identifier)?.token_str;
    let param_name = tokens.next().assert_type(Identifier)?.token_str;

    Ok(ParameterNode::new(param_name, param_type))
}
