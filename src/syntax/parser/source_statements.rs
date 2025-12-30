use std::iter::Peekable;
use std::vec::IntoIter;
use crate::lexer::tokenizer::SourceLines;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::statement::Statement;

pub struct SourceStatements(Vec<Statement>);

impl From<SourceLines> for SourceStatements {
    fn from(source_lines: SourceLines) -> Self {

        let mut statements = Vec::new();
        let mut curr_statement_tokens = Vec::new();

        let mut legal_statement_end = false;

        for mut line in source_lines {
            if line.len() <= 1 {
                continue;
            }

            if legal_statement_end && line[1].is_legal_statement_boundary() {
                statements.push(Statement::new(curr_statement_tokens));

                curr_statement_tokens = line;
            } else {
                let last_token = line.last().expect("Line must have at least one token");

                legal_statement_end = last_token.is_legal_statement_boundary();

                if curr_statement_tokens.len() == 0 {
                    curr_statement_tokens.extend(line.drain(..));
                } else {
                    curr_statement_tokens.extend(line.drain(1..));
                }
            }
        }

        if !curr_statement_tokens.is_empty() {
            statements.push(Statement::new(curr_statement_tokens));
        }

        Self(statements)
    }
}

impl IntoIterator for SourceStatements {
    type Item = Statement;
    type IntoIter = SourceStatementsIter;
    
    fn into_iter(self) -> Self::IntoIter {
        SourceStatementsIter(self.0.into_iter().peekable())
    }
}

pub struct SourceStatementsIter(Peekable<IntoIter<Statement>>);

impl SourceStatementsIter {
    pub fn ast_child_nodes(&mut self, parent_indent_size: usize) -> SyntaxResult<Vec<ASTNode>> {
        let mut child_nodes = Vec::new();

        while let Some(child) = &mut self.0.peek() {

            if child.indent_size <= parent_indent_size {
                break;
            }

            let statement = self.0.next().unwrap();
            child_nodes.push(statement.to_ast_node(self)?)
        }

        Ok(child_nodes)
    }
}

impl Iterator for SourceStatementsIter {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
