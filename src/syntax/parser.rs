use crate::lexer::tokenizer::SourceLines;
use crate::syntax::ast::ast_node::ASTNode::FunctionDef;
use crate::syntax::ast::AST;
use crate::syntax::error::{SyntaxError, SyntaxResult};
use crate::syntax::parser::source_statements::SourceStatements;

pub mod expression;
mod sub_expression;
pub mod statement;
mod function;
mod source_statements;

impl TryFrom<SourceLines> for AST {
    type Error = SyntaxError;

    fn try_from(source_lines: SourceLines) -> SyntaxResult<Self> {

        let statements: SourceStatements = source_lines.into();

        let mut functions = vec![];
        let mut top_level_code = vec![];

        let mut next_statements = statements.into_iter();

        while let Some(statement) = next_statements.next() {
            if statement.len() <= 1 {
                continue;
            }

            match statement.to_ast_node(&mut next_statements)? {
                FunctionDef(function_def_node) => functions.push(function_def_node),
                node => top_level_code.push(node),
            }
        }

        Ok(AST::new(functions, top_level_code))
    }
}
