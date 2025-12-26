use crate::lexer::SourceFileTokens;
use crate::token::Token;

pub struct SourceFileStatements {
    pub file_name: String,
    pub statements: Vec<Vec<Token>>
}

pub fn get_statements(mut source_file_tokens: SourceFileTokens) -> SourceFileStatements {
    let mut statements = Vec::new();
    let mut curr_statement = Vec::new();

    let mut legal_statement_end = false;

    for line in source_file_tokens.lines.iter_mut() {
        if line.len() == 1 {
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

    SourceFileStatements {
        file_name: source_file_tokens.file_name,
        statements,
    }
}
