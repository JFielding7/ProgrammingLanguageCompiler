use crate::lexer::lexer::TokenizedSource;
use crate::lexer::token::Token;

pub type Statement = Vec<Token>;

pub struct ParsedSource {
    pub file_name: String,
    pub statements: Vec<Statement>
}

impl ParsedSource {
    pub fn new(mut lexer: TokenizedSource) -> Self {
        let mut statements = Vec::new();
        let mut curr_statement = Vec::new();

        let mut legal_statement_end = false;

        for line in lexer.lines.iter_mut() {
            if line.len() <= 1 {
                continue;
            }

            if legal_statement_end && line[1].is_legal_statement_boundary() {
                statements.push(curr_statement);

                curr_statement = std::mem::take(line);
            } else {
                let last_token = line.last().unwrap();

                legal_statement_end = last_token.is_legal_statement_boundary();

                if curr_statement.len() == 0 {
                    curr_statement.extend(line.drain(..));
                } else {
                    curr_statement.extend(line.drain(1..));
                }
            }
        }

        if !curr_statement.is_empty() {
            statements.push(curr_statement);
        }

        ParsedSource {
            file_name: lexer.file_name,
            statements,
        }
    }
}

impl IntoIterator for ParsedSource {
    type Item = Statement;
    type IntoIter = std::vec::IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.statements.into_iter()
    }
}

impl<'a> IntoIterator for &'a ParsedSource {
    type Item = &'a Statement;
    type IntoIter = std::slice::Iter<'a, Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.statements.iter()
    }
}
