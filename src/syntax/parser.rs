use crate::lexer::tokenizer::TokenizedLines;
use crate::ast::ast_node::ASTNodeType::FunctionDef;
use crate::ast::AST;
use crate::ast::ast_arena::ASTArena;
use crate::syntax::error::{SyntaxError, SyntaxResult};
use crate::syntax::parser::ast_parser::ASTParser;
use crate::syntax::parser::source_statements::SourceStatements;

pub mod statement;
mod function_def_params;
mod source_statements;
mod expression;
mod token_stream;
mod ast_parser;
mod type_annotation;

impl TryFrom<TokenizedLines> for AST {
    type Error = SyntaxError;

    fn try_from(source_lines: TokenizedLines) -> SyntaxResult<Self> {

        let statements: SourceStatements = source_lines.into();

        let ast_arena = ASTArena::new();

        let mut functions = vec![];
        let mut top_level_code = vec![];

        for node_res in ASTParser::new(statements, ast_arena) {
            let node_id = node_res?;

            if let FunctionDef(_) = ast_arena.lookup(node_id).node_type {
                functions.push(node_id);
            } else {
                top_level_code.push(node_id)
            }
        }

        Ok(AST::new(functions, top_level_code))
    }
}
