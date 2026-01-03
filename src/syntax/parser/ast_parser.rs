use crate::lexer::token::TokenType;
use crate::lexer::token::TokenType::*;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::for_node::ForNode;
use crate::syntax::ast::function_def_node::FunctionDefNode;
use crate::syntax::ast::if_node::{ConditionBlock, IfNode};
use crate::syntax::ast::while_node::WhileNode;
use crate::syntax::error::SyntaxError::IndentTooLarge;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::expression::parse_expression;
use crate::syntax::parser::function_def_params::parse_parameters;
use crate::syntax::parser::source_statements::SourceStatements;
use crate::syntax::parser::statement::Statement;
use crate::syntax::parser::token_stream::TokenStream;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct ASTParser {
    source_statements_iter: Peekable<IntoIter<Statement>>,
}

impl ASTParser {
    pub fn new(source_statements: SourceStatements) -> Self {
        Self {
            source_statements_iter: source_statements.into_iter(),
        }
    }

    fn next_starts_with(&mut self, token_type: TokenType) -> bool {
        self.source_statements_iter
            .peek()
            .is_some_and(|statement| statement.starts_with(token_type))
    }

    fn parse_children(&mut self, statement: &Statement) -> SyntaxResult<Vec<ASTNode>> {
        let indent_size = statement.indent_size;
        let mut children = Vec::new();

        while let Some(child) =self.source_statements_iter.peek() {
            if child.indent_size < indent_size {
                break;
            }

            if indent_size + 1 < child.indent_size {
                return Err(IndentTooLarge(child[0].location.clone()))
            }

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

        let mut token_stream = TokenStream::new(statement.suffix(TOKENS_BEFORE_NAME));

        let name = token_stream.next_identifier()?;
        let params = parse_parameters(&mut token_stream)?;
        let body = self.parse_children(&statement)?;

        Ok(FunctionDefNode::new(name, params, body))
    }

    fn parse_if_statement(&mut self, if_statement: Statement) -> SyntaxResult<IfNode> {
        const TOKENS_BEFORE_COND: usize = 2;

        let if_cond = parse_expression(if_statement.suffix_token_stream(TOKENS_BEFORE_COND))?;
        let if_body = self.parse_children(&if_statement)?;

        let mut condition_blocks = vec![ConditionBlock::new(if_cond, if_body)];

        while self.next_starts_with(Elif) {
            let elif_statement = self.source_statements_iter
                .next()
                .expect("Statement Expected");

            let elif_cond = parse_expression(elif_statement.suffix_token_stream(TOKENS_BEFORE_COND))?;
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

        Ok(IfNode::new(condition_blocks, else_body))
    }

    fn parse_while_loop(&mut self, while_statement: Statement) -> SyntaxResult<WhileNode> {
        const TOKENS_BEFORE_COND: usize = 2;

        let while_cond = parse_expression(while_statement.suffix_token_stream(TOKENS_BEFORE_COND))?;
        let while_body = self.parse_children(&while_statement)?;

        Ok(WhileNode::new(while_cond, while_body))
    }

    fn parse_for_loop(&mut self, for_statement: Statement) -> SyntaxResult<ForNode> {
        const TOKENS_BEFORE_ITEM_IDENT: usize = 2;

        let mut token_stream = TokenStream::new(for_statement.suffix(TOKENS_BEFORE_ITEM_IDENT));

        let item_identifier = token_stream.next_identifier()?;
        token_stream.next_token_of_type(In)?;
        let iterator = parse_expression(token_stream)?;
        let for_body = self.parse_children(&for_statement)?;

        Ok(ForNode::new(item_identifier, iterator, for_body))
    }

    fn next_ast_node(&mut self) -> SyntaxResult<Option<ASTNode>> {
        
        if let Some(statement) = self.source_statements_iter.next() {

            match statement.start_token_type() {
                Fn => Ok(Some(self.parse_function(statement)?.into())),
                If => Ok(Some(self.parse_if_statement(statement)?.into())),
                While => Ok(Some(self.parse_while_loop(statement)?.into())),
                For => Ok(Some(self.parse_for_loop(statement)?.into())),
                _ => Ok(Some(parse_expression(statement.suffix_token_stream(Statement::INDEX_AFTER_INDENT))?)),
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
