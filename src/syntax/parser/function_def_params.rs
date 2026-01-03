use crate::lexer::token::TokenType::{CloseParen, Comma, Identifier, OpenParen};
use crate::syntax::ast::function_def_node::Parameter;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::token_stream::TokenStream;

pub fn parse_parameters(token_stream: &mut TokenStream) -> SyntaxResult<Vec<Parameter>> {
    token_stream.next_token_of_type(OpenParen)?;

    let mut params = Vec::new();

    if token_stream.required_next_matches(CloseParen)? {
        return Ok(params);
    }

    params.push(parse_parameter(token_stream)?);

    while token_stream.required_next_matches(Comma)? {
        token_stream.next();
        params.push(parse_parameter(token_stream)?);
    }

    token_stream.next_token_of_type(CloseParen)?;

    Ok(params)
}

fn parse_parameter(token_stream: &mut TokenStream) -> SyntaxResult<Parameter> {
    let param_type = token_stream.next_token_of_type(Identifier)?.token_str.clone();
    let param_name = token_stream.next_token_of_type(Identifier)?.token_str.clone();

    Ok(Parameter::new(param_name, param_type))
}
