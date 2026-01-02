use crate::lexer::token::TokenType::{CloseParen, Comma, Identifier, OpenParen};
use crate::syntax::ast::parameter_node::ParameterNode;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::token_stream::TokenStream;

pub fn parse_function_name(token_stream: &mut TokenStream) -> SyntaxResult<String> {
    token_stream.next_token_of_type(Identifier).map(|token| token.token_str.clone())
}

pub fn parse_parameters(token_stream: &mut TokenStream) -> SyntaxResult<Vec<ParameterNode>> {
    token_stream.next_token_of_type(OpenParen)?;

    let mut params = Vec::new();

    if token_stream.expect_next_token_type(CloseParen)? {
        return Ok(params);
    }

    params.push(parse_parameter(token_stream)?);

    while token_stream.expect_next_token_type(Comma)? {
        params.push(parse_parameter(token_stream)?);
    }

    token_stream.next_token_of_type(CloseParen)?;

    Ok(params)
}

fn parse_parameter(token_stream: &mut TokenStream) -> SyntaxResult<ParameterNode> {
    let param_type = token_stream.next_token_of_type(Identifier)?.token_str.clone();
    let param_name = token_stream.next_token_of_type(Identifier)?.token_str.clone();

    Ok(ParameterNode::new(param_name, param_type))
}
