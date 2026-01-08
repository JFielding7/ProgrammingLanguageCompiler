use crate::ast::arena_ast::{ASTNodeId, AST};
use crate::ast::ast_node::SpannableASTNode;
use crate::ast::for_node::ForNode;
use crate::ast::function_def_node::FunctionDefNode;
use crate::ast::if_node::{ConditionBlock, IfNode};
use crate::ast::while_node::WhileNode;
use crate::error::spanned_error::SpannableError;
use crate::lexer::token::TokenType::*;
use crate::lexer::token::TokenType;
use crate::lexer::tokenizer::TokenizedLines;
use crate::syntax::error::SyntaxError::IndentTooLarge;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::expression::ExpressionParser;
use crate::syntax::parser::function_signature::{parse_parameters, parse_return_type};
use crate::syntax::parser::source_statements::SourceStatements;
use crate::syntax::parser::statement::Statement;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct ASTParser {
    statements_iter: Peekable<IntoIter<Statement>>,
    ast: AST,
}

impl ASTParser {
    pub fn new(statements: SourceStatements) -> Self {
        Self {
            statements_iter: statements.into_iter(),
            ast: AST::new()
        }
    }

    fn next_starts_with(&mut self, token_type: TokenType) -> bool {
        self.statements_iter
            .peek()
            .is_some_and(|statement| statement.token_after_indent_matches(token_type))
    }

    fn parse_children(&mut self, statement: &Statement) -> SyntaxResult<Vec<ASTNodeId>> {
        let indent_size = statement.indent_size;
        let mut children = Vec::new();

        while let Some(child) =self.statements_iter.peek() {
            if child.indent_size < indent_size {
                break;
            }

            if indent_size + 1 < child.indent_size {
                return Err(IndentTooLarge.at(child[0].span))
            }

            if let Some(child) = self.parse_top_level()? {
                children.push(child);
            }
        }

        Ok(children)
    }

    fn parse_function(&mut self, func_def_statement: &Statement) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_NAME: usize = 2;

        let mut token_stream = func_def_statement.suffix_stream(TOKENS_BEFORE_NAME);

        let name = token_stream.expect_next_identifier()?;
        let params = parse_parameters(&mut token_stream)?;
        let body = self.parse_children(&func_def_statement)?;

        let func_def_node = FunctionDefNode::new(name, params, body)
            .at(func_def_statement.full_span()).annotate_type(parse_return_type(&mut token_stream)?);
        
        Ok(self.ast.add_node(func_def_node))
    }

    fn parse_if_statement(&mut self, if_statement: &Statement) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_COND: usize = 2;

        let if_cond = ExpressionParser::parse(
            &mut if_statement.suffix_stream(TOKENS_BEFORE_COND),
            &mut self.ast
        )?;
        let if_body = self.parse_children(&if_statement)?;

        let mut condition_blocks = vec![ConditionBlock::new(if_cond, if_body)];

        while self.next_starts_with(Elif) {
            let elif_statement = self.statements_iter
                .next()
                .expect("Statement Expected");

            let elif_cond = ExpressionParser::parse(
                &mut elif_statement.suffix_stream(TOKENS_BEFORE_COND),
                &mut self.ast
            )?;
            let elif_body = self.parse_children(&elif_statement)?;

            condition_blocks.push(ConditionBlock::new(elif_cond, elif_body));
        }

        let else_body = if self.next_starts_with(Else) {
            let else_statement = self.statements_iter
                .next()
                .expect("Statement Expected");

            Some(self.parse_children(&else_statement)?)
        } else {
            None
        };

        let if_node = IfNode::new(condition_blocks, else_body)
            .at(if_statement.full_span());

        Ok(self.ast.add_node(if_node))
    }

    fn parse_while_loop(&mut self, while_statement: &Statement) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_COND: usize = 2;

        let while_cond = ExpressionParser::parse(
            &mut while_statement.suffix_stream(TOKENS_BEFORE_COND),
            &mut self.ast
        )?;
        let while_body = self.parse_children(&while_statement)?;

        let while_node = WhileNode::new(while_cond, while_body)
            .at(while_statement.full_span());

        Ok(self.ast.add_node(while_node))
    }

    fn parse_for_loop(&mut self, for_statement: &Statement) -> SyntaxResult<ASTNodeId> {
        const TOKENS_BEFORE_ITEM_IDENT: usize = 2;

        let mut token_stream = for_statement.suffix_stream(TOKENS_BEFORE_ITEM_IDENT);

        let item_identifier = token_stream.expect_next_identifier()?;
        token_stream.expect_next_token(In)?;
        let iterator = ExpressionParser::parse(&mut token_stream, &mut self.ast)?;
        let for_body = self.parse_children(&for_statement)?;

        let node = ForNode::new(item_identifier, iterator, for_body)
            .at(for_statement.full_span());

        Ok(self.ast.add_node(node))
    }

    fn parse_top_level(&mut self) -> SyntaxResult<Option<ASTNodeId>> {
        
        if let Some(statement) = &self.statements_iter.next() {

            Ok(Some(match statement.token_after_indent_type() {
                Fn => self.parse_function(statement)?,
                If => self.parse_if_statement(statement)?,
                While => self.parse_while_loop(statement)?,
                For => self.parse_for_loop(statement)?,
                _ => ExpressionParser::parse(
                    &mut statement.suffix_stream(Statement::INDEX_AFTER_INDENT),
                    &mut self.ast
                )?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn generate_ast(source_lines: TokenizedLines) -> SyntaxResult<AST> {

        let statements: SourceStatements = source_lines.into();
        let mut parser = Self::new(statements);

        while let Some(node_id) = parser.parse_top_level()? {
            parser.ast.add_top_level_node(node_id);
        }

        Ok(parser.ast)
    }
}

