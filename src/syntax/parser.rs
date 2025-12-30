use crate::lexer::token::TokenType::Fn;
use crate::lexer::tokenizer::SourceLines;
use crate::syntax::ast::ast_node::ASTNode::FunctionDef;
use crate::syntax::ast::AST;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::error::{SyntaxError, SyntaxResult};
use crate::syntax::parser::expression::ExpressionParser;
use crate::syntax::parser::function_signature::{parse_function_name, parse_parameters};
use crate::syntax::parser::source_statements::{SourceStatements, SourceStatementsIter};
use crate::syntax::parser::statement::{Statement, StatementParser};

pub mod expression;
mod sub_expression;
pub mod statement;
mod function_signature;
mod source_statements;

impl TryFrom<SourceLines> for AST {
    type Error = SyntaxError;

    fn try_from(source_lines: SourceLines) -> SyntaxResult<Self> {

        let statements: SourceStatements = source_lines.into();

        let mut parser = Parser::new(statements);

        let mut functions = vec![];
        let mut top_level_code = vec![];

        while let Some(node) = parser.next_ast_node()? {

            if let FunctionDef(function_def_node) = node {
                functions.push(function_def_node);
            } else {
                top_level_code.push(node)
            }
        }

        Ok(AST::new(functions, top_level_code))
    }
}

struct Parser {
    source_statements_iter: SourceStatementsIter,
}

impl Parser {
    fn new(source_statements: SourceStatements) -> Self {
        Self {
            source_statements_iter: source_statements.into_iter(),
        }
    }

    fn parse_function(
        &mut self,
        statement: Statement,
    ) -> SyntaxResult<FunctionDefNode> {
        const TOKENS_BEFORE_NAME: usize = 2;

        let mut statement_parser = StatementParser::new(&statement);
        statement_parser.skip(TOKENS_BEFORE_NAME);

        let name = parse_function_name(&mut statement_parser)?;
        let params = parse_parameters(&mut statement_parser)?;
        let mut body = vec![];
        
        while self.source_statements_iter.next_is_child(statement.indent_size) {
            if let Some(child) = self.next_ast_node()? {
                body.push(child);
            }
        }

        Ok(FunctionDefNode::new(name, params, body))
    }
    
    fn next_ast_node(&mut self) -> SyntaxResult<Option<ASTNode>> {

        let statement_iter = &mut self.source_statements_iter;

        if let Some(statement) = statement_iter.next() {

            match statement[Statement::INDEX_AFTER_INDENT].token_type {
                Fn => Ok(Some(self.parse_function(statement)?.into())),
                _ => Ok(Some(ExpressionParser::parse(&statement)?)),
            }
        } else {
            Ok(None)
        }
    }
}
