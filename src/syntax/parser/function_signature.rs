use crate::lexer::token::TokenType::{CloseParen, Comma, Identifier, OpenParen};
use crate::syntax::ast::parameter_node::ParameterNode;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::statement_parser::StatementParser;

pub fn parse_function_name(statement_parser: &mut StatementParser) -> SyntaxResult<String> {
    statement_parser.next_token_of_type(Identifier).map(|token| token.token_str.clone())
}

pub fn parse_parameters(statement_parser: &mut StatementParser) -> SyntaxResult<Vec<ParameterNode>> {
    statement_parser.next_token_of_type(OpenParen)?;

    let mut params = Vec::new();

    if statement_parser.cmp_next_token_type(CloseParen)? {
        return Ok(params);
    }

    params.push(parse_parameter(statement_parser)?);

    while statement_parser.cmp_next_token_type(Comma)? {
        params.push(parse_parameter(statement_parser)?);
    }

    statement_parser.next_token_of_type(CloseParen)?;

    Ok(params)
}

fn parse_parameter(statement_parser: &mut StatementParser) -> SyntaxResult<ParameterNode> {
    let param_type = statement_parser.next_token_of_type(Identifier)?.token_str.clone();
    let param_name = statement_parser.next_token_of_type(Identifier)?.token_str.clone();

    Ok(ParameterNode::new(param_name, param_type))
}
