use crate::lexer::token::TokenType::{CloseParen, Comma, Identifier, OpenParen};
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::ast::parameter_node::ParameterNode;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::source_statements::SourceStatementsIter;
use crate::syntax::parser::statement::{Statement, StatementParser};


pub fn parse_function_def(
    statement: Statement,
    next_statements: &mut SourceStatementsIter
) -> SyntaxResult<FunctionDefNode> {

    let indent_size = statement.indent_size;
    let mut statement_parser = StatementParser::new(&statement);
    statement_parser.skip(2);

    let name = parse_function_name(&mut statement_parser)?;
    let params = parse_parameters(&mut statement_parser)?;
    let body = next_statements.ast_child_nodes(indent_size)?;

    Ok(FunctionDefNode::new(name, params, body))
}

fn parse_function_name(statement_parser: &mut StatementParser) -> SyntaxResult<String> {
    statement_parser.next_token_of_type(Identifier).map(|token| token.token_str)
}

fn parse_parameters(statement_parser: &mut StatementParser) -> SyntaxResult<Vec<ParameterNode>> {
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
    let param_type = statement_parser.next_token_of_type(Identifier)?.token_str;
    let param_name = statement_parser.next_token_of_type(Identifier)?.token_str;

    Ok(ParameterNode::new(param_name, param_type))
}
