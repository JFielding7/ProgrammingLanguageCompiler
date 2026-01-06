use crate::lexer::token::TokenType::{CloseParen, Colon, Comma, Identifier, OpenParen};
use crate::ast::function_def_node::Parameter;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::token_stream::TokenStream;
use crate::syntax::parser::type_annotation::parse_type_annotation;

pub fn parse_parameters(token_stream: &mut TokenStream) -> SyntaxResult<Vec<Parameter>> {
    token_stream.expect_next_token(OpenParen)?;

    let mut params = Vec::new();

    if token_stream.peek_matches(CloseParen) {
        return Ok(params);
    }

    params.push(parse_parameter(token_stream)?);

    while token_stream.peek_matches(Comma) {
        token_stream.next();
        params.push(parse_parameter(token_stream)?);
    }

    token_stream.expect_next_token(CloseParen)?;

    Ok(params)
}

fn parse_parameter(token_stream: &mut TokenStream) -> SyntaxResult<Parameter> {
    let param_name = token_stream.expect_next_token(Identifier)?.token_str.clone();
    token_stream.expect_next_token(Colon)?;
    let type_annotation = parse_type_annotation(token_stream)?;

    Ok(Parameter::new(param_name, type_annotation))
}
