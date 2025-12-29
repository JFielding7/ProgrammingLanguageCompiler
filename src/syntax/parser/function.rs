use crate::error::compiler_error::Result;
use crate::lexer::token::TokenType::{CloseParen, Comma, Identifier, OpenParen};
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::ast::parameter_node::ParameterNode;
use crate::syntax::parser::source_statements::SourceStatementsIter;
use crate::syntax::parser::statement::{Statement, StatementParser};


pub fn parse_function_def(
    statement: Statement,
    next_statements: &mut SourceStatementsIter
) -> Result<FunctionDefNode> {

    let indent_size = statement.indent_size;
    let mut tokens = StatementParser::new(&statement);

    let name = parse_function_name(&mut tokens)?;
    let params = parse_parameters(&mut tokens)?;
    let body = next_statements.ast_child_nodes(indent_size)?;

    Ok(FunctionDefNode::new(name, params, body))
}

fn parse_function_name(statement_parser: &mut StatementParser) -> Result<String> {
    statement_parser.next_token_of_type(Identifier).map(|token| token.token_str)
}

fn parse_parameters(statement_parser: &mut StatementParser) -> Result<Vec<ParameterNode>> {
    statement_parser.next_token_of_type(OpenParen)?;

    let mut params = Vec::new();

    if let Some(token) = statement_parser.peek() {
        if *token == CloseParen {
            return Ok(params);
        }
    }

    params.push(parse_parameter(statement_parser)?);

    while statement_parser.is_next_token_of_type(Comma) {
        params.push(parse_parameter(statement_parser)?);
    }

    statement_parser.next_token_of_type(CloseParen)?;

    Ok(params)
}

fn parse_parameter(statement_parser: &mut StatementParser) -> Result<ParameterNode> {
    let param_type = statement_parser.next_token_of_type(Identifier)?.token_str;
    let param_name = statement_parser.next_token_of_type(Identifier)?.token_str;

    Ok(ParameterNode::new(param_name, param_type))
}
