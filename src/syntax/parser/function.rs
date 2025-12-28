use std::iter::{Peekable, Skip};
use std::vec::IntoIter;
use crate::error::compiler_error::Result;
use crate::lexer::token::{Token, TokenOpt};
use crate::lexer::token::TokenType::{CloseParen, Comma, Identifier, OpenParen};
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::ast::parameter_node::ParameterNode;
use crate::syntax::parser::statement::Statement;

type TokenIter = Peekable<Skip<IntoIter<Token>>>;

pub fn parse_function_def(statement: Statement) -> Result<FunctionDefNode> {
    let mut tokens = statement.into_iter().skip(2).peekable();

    let name = parse_function_name(&mut tokens)?;
    let params = parse_parameters(&mut tokens)?;

    Ok(FunctionDefNode::new(name, params))
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
