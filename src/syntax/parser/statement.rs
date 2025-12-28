use std::ops::Index;
use crate::error::compiler_error::Result;
use crate::lexer::token::Token;
use crate::lexer::tokenizer::SourceLines;
use std::vec::IntoIter;

pub struct Statement {
    pub tokens: Vec<Token>,
}

impl Statement {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn indent_size(&self) -> Result<usize> {
        self.tokens[0].indent_size()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

pub struct SourceStatements {
    pub file_name: String,
    pub statements: Vec<Statement>
}

impl SourceStatements {
    fn new(file_name: String, statements: Vec<Statement>) -> Self {
        SourceStatements {
            file_name,
            statements,
        }
    }
}

impl From<SourceLines> for SourceStatements {
    fn from(source_lines: SourceLines) -> Self {
        let SourceLines { file_name, lines } = source_lines;

        let mut statements = Vec::new();
        let mut curr_statement_tokens = Vec::new();

        let mut legal_statement_end = false;

        for mut line in lines {
            if line.len() <= 1 {
                continue;
            }

            if legal_statement_end && line[1].is_legal_statement_boundary() {
                statements.push(Statement::new(curr_statement_tokens));

                curr_statement_tokens = line;
            } else {
                let last_token = line.last().unwrap();

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

        Self::new(file_name, statements)
    }
}

impl IntoIterator for Statement {
    type Item = Token;
    type IntoIter = IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl IntoIterator for SourceStatements {
    type Item = Statement;
    type IntoIter = IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.statements.into_iter()
    }
}

impl Index<usize> for Statement {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}
