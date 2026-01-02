use crate::lexer::tokenizer::SourceLines;
use crate::syntax::ast::ast_node::ASTNode::FunctionDef;
use crate::syntax::ast::AST;
use crate::syntax::error::{SyntaxError, SyntaxResult};
use crate::syntax::parser::ast_parser::ASTParser;
use crate::syntax::parser::source_statements::SourceStatements;

pub mod statement;
mod function_signature;
mod source_statements;
mod expression;
mod token_stream;
mod ast_parser;

impl TryFrom<SourceLines> for AST {
    type Error = SyntaxError;

    fn try_from(source_lines: SourceLines) -> SyntaxResult<Self> {

        let statements: SourceStatements = source_lines.into();

        let mut functions = vec![];
        let mut top_level_code = vec![];

        for node_res in ASTParser::new(statements) {
            let node = node_res?;

            if let FunctionDef(function_def_node) = node {
                functions.push(function_def_node);
            } else {
                top_level_code.push(node)
            }
        }

        Ok(AST::new(functions, top_level_code))
    }
}
