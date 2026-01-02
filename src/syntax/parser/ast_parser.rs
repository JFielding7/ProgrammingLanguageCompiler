use crate::lexer::token::TokenType::{Elif, Else, Fn, If};
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::ast::if_node::{ConditionBlock, IfNode};
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::expression::parse_expression;
use crate::syntax::parser::function_signature::{parse_function_name, parse_parameters};
use crate::syntax::parser::source_statements::{SourceStatements, SourceStatementsIter};
use crate::syntax::parser::statement::Statement;
use crate::syntax::parser::statement_parser::StatementParser;

pub struct ASTParser {
    source_statements_iter: SourceStatementsIter,
}

impl ASTParser {
    pub(crate) fn new(source_statements: SourceStatements) -> Self {
        Self {
            source_statements_iter: source_statements.into_iter(),
        }
    }

    fn parse_children(&mut self, parent_indent: usize) -> SyntaxResult<Vec<ASTNode>> {
        let mut children = Vec::new();

        while self.source_statements_iter.next_is_child(parent_indent) {
            if let Some(child) = self.next_ast_node()? {
                children.push(child);
            }
        }

        Ok(children)
    }

    fn parse_function(
        &mut self,
        statement: Statement,
    ) -> SyntaxResult<FunctionDefNode> {
        const TOKENS_BEFORE_NAME: usize = 2;

        let parent_indent = statement.indent_size;
        let mut statement_parser = StatementParser::from_suffix(&statement, TOKENS_BEFORE_NAME);

        let name = parse_function_name(&mut statement_parser)?;
        let params = parse_parameters(&mut statement_parser)?;
        let body = self.parse_children(parent_indent)?;

        Ok(FunctionDefNode::new(name, params, body))
    }

    fn parse_if_statement(&mut self, statement: Statement) -> SyntaxResult<IfNode> {
        const TOKENS_BEFORE_COND: usize = 2;

        let if_cond = parse_expression(&statement, TOKENS_BEFORE_COND)?;
        let if_body = self.parse_children(statement.indent_size)?;

        let mut condition_blocks = vec![ConditionBlock::new(if_cond, if_body)];

        while self.source_statements_iter.next_starts_with(Elif) {
            let statement = self.source_statements_iter
                .next()
                .expect("Statement Expected");

            let elif_cond = parse_expression(&statement, TOKENS_BEFORE_COND)?;
            let elif_body = self.parse_children(statement.indent_size)?;

            condition_blocks.push(ConditionBlock::new(elif_cond, elif_body));
        }

        let else_body = if self.source_statements_iter.next_starts_with(Else) {
            let statement = self.source_statements_iter
                .next()
                .expect("Statement Expected");

            Some(self.parse_children(statement.indent_size)?)
        } else {
            None
        };

        Ok(IfNode::new(condition_blocks, else_body))
    }

    fn next_ast_node(&mut self) -> SyntaxResult<Option<ASTNode>> {

        let statement_iter = &mut self.source_statements_iter;

        if let Some(statement) = statement_iter.next() {

            match statement[Statement::INDEX_AFTER_INDENT].token_type {
                Fn => Ok(Some(self.parse_function(statement)?.into())),
                If => Ok(Some(self.parse_if_statement(statement)?.into())),
                _ => Ok(Some(parse_expression(&statement, Statement::INDEX_AFTER_INDENT)?)),
            }
        } else {
            Ok(None)
        }
    }
}

impl Iterator for ASTParser {
    type Item = SyntaxResult<ASTNode>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_ast_node().transpose()
    }
}
