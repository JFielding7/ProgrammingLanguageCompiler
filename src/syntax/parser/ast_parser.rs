use crate::lexer::token::TokenType::*;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::for_node::ForNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::ast::if_node::{ConditionBlock, IfNode};
use crate::syntax::ast::while_node::WhileNode;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::expression::{parse_expression, parse_expression_from_token_stream};
use crate::syntax::parser::function_def_params::parse_parameters;
use crate::syntax::parser::source_statements::{SourceStatements, SourceStatementsIter};
use crate::syntax::parser::statement::Statement;
use crate::syntax::parser::token_stream::TokenStream;

pub struct ASTParser {
    source_statements_iter: SourceStatementsIter,
}

impl ASTParser {
    pub fn new(source_statements: SourceStatements) -> Self {
        Self {
            source_statements_iter: source_statements.into_iter(),
        }
    }

    fn parse_children(&mut self, statement: &Statement) -> SyntaxResult<Vec<ASTNode>> {
        let mut children = Vec::new();

        while self.source_statements_iter.next_is_child(statement.indent_size) {
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

        let mut token_stream = TokenStream::from_statement_suffix(&statement, TOKENS_BEFORE_NAME);

        let name = token_stream.next_identifier()?;
        let params = parse_parameters(&mut token_stream)?;
        let body = self.parse_children(&statement)?;

        Ok(FunctionDefNode::new(name, params, body))
    }

    fn parse_if_statement(&mut self, statement: Statement) -> SyntaxResult<IfNode> {
        const TOKENS_BEFORE_COND: usize = 2;

        let if_cond = parse_expression(&statement, TOKENS_BEFORE_COND)?;
        let if_body = self.parse_children(&statement)?;

        let mut condition_blocks = vec![ConditionBlock::new(if_cond, if_body)];

        while self.source_statements_iter.next_starts_with(Elif) {
            let statement = self.source_statements_iter
                .next()
                .expect("Statement Expected");

            let elif_cond = parse_expression(&statement, TOKENS_BEFORE_COND)?;
            let elif_body = self.parse_children(&statement)?;

            condition_blocks.push(ConditionBlock::new(elif_cond, elif_body));
        }

        let else_body = if self.source_statements_iter.next_starts_with(Else) {
            let statement = self.source_statements_iter
                .next()
                .expect("Statement Expected");

            Some(self.parse_children(&statement)?)
        } else {
            None
        };

        Ok(IfNode::new(condition_blocks, else_body))
    }

    fn parse_while_loop(&mut self, while_statement: Statement) -> SyntaxResult<WhileNode> {
        const TOKENS_BEFORE_COND: usize = 2;

        let while_cond = parse_expression(&while_statement, TOKENS_BEFORE_COND)?;
        let while_body = self.parse_children(&while_statement)?;

        Ok(WhileNode::new(while_cond, while_body))
    }

    fn parse_for_loop(&mut self, for_statement: Statement) -> SyntaxResult<ForNode> {
        const TOKENS_BEFORE_ITEM_IDENT: usize = 2;

        let mut token_stream = TokenStream::from_statement_suffix(&for_statement, TOKENS_BEFORE_ITEM_IDENT);

        let item_identifier = token_stream.next_identifier()?;
        token_stream.next_token_of_type(In)?;
        let iterator = parse_expression_from_token_stream(&mut token_stream)?;
        let for_body = self.parse_children(&for_statement)?;

        Ok(ForNode::new(item_identifier, iterator, for_body))
    }

    fn next_ast_node(&mut self) -> SyntaxResult<Option<ASTNode>> {

        let statement_iter = &mut self.source_statements_iter;

        if let Some(statement) = statement_iter.next() {

            match statement[Statement::INDEX_AFTER_INDENT].token_type {
                Fn => Ok(Some(self.parse_function(statement)?.into())),
                If => Ok(Some(self.parse_if_statement(statement)?.into())),
                While => Ok(Some(self.parse_while_loop(statement)?.into())),
                For => Ok(Some(self.parse_for_loop(statement)?.into())),
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
