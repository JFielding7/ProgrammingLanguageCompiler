use crate::ast::ast_arena::{ASTArena, ASTNodeId};
use crate::ast::ast_node::ASTNode;
use crate::ast::for_node::ForNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::{ConditionBlock, IfNode};
use crate::ast::while_node::WhileNode;
use crate::error::spanned_error::WithSpan;
use crate::lexer::token::TokenType;
use crate::lexer::token::TokenType::*;
use crate::syntax::error::SyntaxErrorType::IndentTooLarge;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::expression::ExpressionParser;
use crate::syntax::parser::function_def_params::parse_parameters;
use crate::syntax::parser::source_statements::SourceStatements;
use crate::syntax::parser::statement::Statement;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct ASTParser {
    source_statements_iter: Peekable<IntoIter<Statement>>,
    ast_arena: ASTArena,
}

impl ASTParser {
    pub fn new(source_statements: SourceStatements, ast_arena: ASTArena) -> Self {
        Self {
            source_statements_iter: source_statements.into_iter(),
            ast_arena
        }
    }

    fn next_starts_with(&mut self, token_type: TokenType) -> bool {
        self.source_statements_iter
            .peek()
            .is_some_and(|statement| statement.starts_with(token_type))
    }

    fn parse_children(&mut self, statement: &Statement) -> SyntaxResult<Vec<ASTNodeId>> {
        let indent_size = statement.indent_size;
        let mut children = Vec::new();

        while let Some(child) =self.source_statements_iter.peek() {
            if child.indent_size < indent_size {
                break;
            }

            if indent_size + 1 < child.indent_size {
                return Err(IndentTooLarge.at(child[0].span.clone()))
            }

            if let Some(child) = self.next_ast_node_id()? {
                children.push(child);
            }
        }

        Ok(children)
    }

    fn parse_function(&mut self, statement: &Statement) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_NAME: usize = 2;

        let mut token_stream = statement.suffix_token_stream(TOKENS_BEFORE_NAME);

        let name = token_stream.expect_next_identifier()?;
        let params = parse_parameters(&mut token_stream)?;
        let body = self.parse_children(&statement)?;

        let func_def_node = FunctionDefNode::new(name, params, body);

        Ok(self.ast_arena.add_with_span(func_def_node, statement.full_span()))
    }

    fn parse_if_statement(&mut self, if_statement: &Statement) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_COND: usize = 2;

        let if_cond = ExpressionParser::parse(&mut if_statement.suffix_token_stream(TOKENS_BEFORE_COND), &mut self.ast_arena)?;
        let if_body = self.parse_children(&if_statement)?;

        let mut condition_blocks = vec![ConditionBlock::new(if_cond, if_body)];

        while self.next_starts_with(Elif) {
            let elif_statement = self.source_statements_iter
                .next()
                .expect("Statement Expected");

            let elif_cond = ExpressionParser::parse(&mut elif_statement.suffix_token_stream(TOKENS_BEFORE_COND), &mut self.ast_arena)?;
            let elif_body = self.parse_children(&elif_statement)?;

            condition_blocks.push(ConditionBlock::new(elif_cond, elif_body));
        }

        let else_body = if self.next_starts_with(Else) {
            let else_statement = self.source_statements_iter
                .next()
                .expect("Statement Expected");

            Some(self.parse_children(&else_statement)?)
        } else {
            None
        };

        let if_node = IfNode::new(condition_blocks, else_body);

        Ok(self.ast_arena.add_with_span(if_node, if_statement.full_span()))
    }

    fn parse_while_loop(&mut self, while_statement: &Statement) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_COND: usize = 2;

        let while_cond = ExpressionParser::parse(&mut while_statement.suffix_token_stream(TOKENS_BEFORE_COND), &mut self.ast_arena)?;
        let while_body = self.parse_children(&while_statement)?;

        let while_node = WhileNode::new(while_cond, while_body);

        Ok(self.ast_arena.add_with_span(while_node, while_statement.full_span()))
    }

    fn parse_for_loop(&mut self, for_statement: &Statement) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_ITEM_IDENT: usize = 2;

        let mut token_stream = for_statement.suffix_token_stream(TOKENS_BEFORE_ITEM_IDENT);

        let item_identifier = token_stream.expect_next_identifier()?;
        token_stream.expect_next_token(In)?;
        let iterator = ExpressionParser::parse(&mut token_stream, &mut self.ast_arena)?;
        let for_body = self.parse_children(&for_statement)?;

        let node = ForNode::new(item_identifier, iterator, for_body);

        Ok(self.ast_arena.add_with_span(node, for_statement.full_span()))
    }

    fn next_ast_node_id(&mut self) -> SyntaxResult<Option<ASTNodeId>> {
        
        if let Some(statement) = &self.source_statements_iter.next() {

            Ok(Some(match statement.start_token_type() {
                Fn => self.parse_function(statement)?,
                If => self.parse_if_statement(statement)?,
                While => self.parse_while_loop(statement)?,
                For => self.parse_for_loop(statement)?,
                _ => ExpressionParser::parse(&mut statement.suffix_token_stream(Statement::INDEX_AFTER_INDENT), &mut self.ast_arena)?,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Iterator for ASTParser {
    type Item = SyntaxResult<ASTNodeId>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_ast_node_id().transpose()
    }
}
