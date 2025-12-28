use crate::error::compiler_error::CompilerError;
use crate::error::compiler_error::Result;
use crate::lexer::token::TokenType::Fn;
use crate::lexer::tokenizer::SourceLines;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::ast_node::ASTNode::FunctionDef;
use crate::syntax::ast::AST;
use crate::syntax::parser::expression::ExpressionParser;
use crate::syntax::parser::function::parse_function_def;
use crate::syntax::parser::statement::{SourceStatements, Statement};

pub mod expression;
mod sub_expression;
pub mod statement;
mod function;

fn next_ast_node(statement: Statement) -> Result<ASTNode> {
    match statement[1].token_type {
        Fn => Ok(parse_function_def(statement)?.into()),
        _ => Ok(ExpressionParser::parse(&statement)?),
    }
}

impl TryFrom<SourceLines> for AST {
    type Error = CompilerError;

    fn try_from(source_lines: SourceLines) -> Result<Self> {

        let SourceStatements { statements, file_name } = source_lines.into();

        let mut functions = vec![];
        let mut top_level_code = vec![];
        let mut prev_indent = 0;

        for statement in statements {
            if statement.len() <= 1 {
                continue;
            }

            let indent = statement.indent_size()?;
            if indent > prev_indent + 1 {

            }

            let node: ASTNode = statement.try_into()?;

            match node {
                FunctionDef(node) => {
                    functions.push(node);
                },
                _ => {
                    // TODO
                }
            }
        }

        Ok(AST::new(file_name, functions, top_level_code))
    }
}
