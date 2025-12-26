use crate::error::compiler_error::CompilerError::InvalidToken;
use crate::error::compiler_error::{ErrorInfo, Result};
use crate::token::TokenType::Indent;
use crate::token::{Token, TokenType};
use logos::Logos;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct SourceFileTokens {
    pub file_name: String,
    pub lines: Vec<Vec<Token>>
}

fn read_source_file(name: &String) -> Result<String> {
    let mut file = File::open(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn tokenize_line(line_num: usize, line: &str) -> Result<Vec<Token>> {
    let indent_spaces = line.chars().take_while(|&c| c == ' ').count();

    let error_info = ErrorInfo::new(line_num, 0, indent_spaces);
    let mut tokens = vec![Token::new(Indent(indent_spaces), "".to_string(), error_info)];

    let mut lexer = TokenType::lexer(&line);

    while let Some(next_token) = lexer.next() {
        let span = lexer.span();

        let token_type = next_token
            .map_err(|_| InvalidToken(ErrorInfo::new(line_num, span.start, span.end)))?;
        let error_info = ErrorInfo::new(line_num, span.start, span.end);

        tokens.push(Token::new(
            token_type,
            lexer.slice().to_string(),
            error_info
        ));
    }

    Ok(tokens)
}

pub fn tokenize_file(file_name: String) -> Result<SourceFileTokens> {
    let lines = read_source_file(&file_name)?
        .split("\n")
        .enumerate()
        .map(|(i, line)| tokenize_line(i + 1, line))
        .collect::<Result<Vec<Vec<Token>>>>()?;

    Ok(SourceFileTokens {
        lines,
        file_name,
    })
}
