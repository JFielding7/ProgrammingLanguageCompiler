use crate::lexer::tokenizer::SourceLines;
use crate::syntax::parser::statement::Statement;
use std::iter::Peekable;
use std::vec::IntoIter;
use crate::lexer::token::TokenType;

pub struct SourceStatements {
    statements: Vec<Statement>
}

impl From<SourceLines> for SourceStatements {
    fn from(source_lines: SourceLines) -> Self {

        let mut statements = Vec::new();
        let mut curr_statement_tokens = Vec::new();

        let mut legal_statement_end = false;

        for mut line in source_lines {
            if line.len() <= Statement::INDEX_AFTER_INDENT {
                continue;
            }

            if legal_statement_end && line[Statement::INDEX_AFTER_INDENT].is_legal_statement_boundary() {
                statements.push(Statement::new(curr_statement_tokens));

                curr_statement_tokens = line;
            } else {
                let last_token = line.last().expect("Line must have at least one token");

                legal_statement_end = last_token.is_legal_statement_boundary();

                if curr_statement_tokens.len() == 0 {
                    curr_statement_tokens.extend(line.drain(..));
                } else {
                    curr_statement_tokens.extend(line.drain(Statement::INDEX_AFTER_INDENT..));
                }
            }
        }

        if !curr_statement_tokens.is_empty() {
            statements.push(Statement::new(curr_statement_tokens));
        }

        Self { statements }
    }
}

impl IntoIterator for SourceStatements {
    type Item = Statement;
    type IntoIter = SourceStatementsIter;
    
    fn into_iter(self) -> Self::IntoIter {
        SourceStatementsIter::new(self)
    }
}

pub struct SourceStatementsIter {
    iter: Peekable<IntoIter<Statement>>
}

impl SourceStatementsIter {
    fn new(source_statements: SourceStatements) -> Self {
        Self { 
            iter: source_statements.statements.into_iter().peekable() 
        }
    }
    
    pub fn next_is_child(&mut self, parent_indent_size: usize) -> bool {

        if let Some(child) = &mut self.iter.peek() {
            child.indent_size > parent_indent_size
        } else {
            false
        }
    }

    pub fn next_starts_with(&mut self, token_type: TokenType) -> bool {
        if let Some(statement) = self.iter.peek() {
            statement.starts_with(token_type)
        } else {
            false
        }
    }
}

impl Iterator for SourceStatementsIter {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
