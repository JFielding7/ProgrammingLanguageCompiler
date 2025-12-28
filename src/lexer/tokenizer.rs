use crate::error::compiler_error::CompilerError::{InvalidIndent, InvalidToken};
use crate::error::compiler_error::{ErrorInfo, Result};
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use logos::Logos;
use std::fs::File;
use std::io::Read;


type Line = Vec<Token>;

#[derive(Debug)]
pub struct SourceLines {
    pub file_name: String,
    pub lines: Vec<Line>
}

impl SourceLines {
    pub fn tokenize_file(file_name: String) -> Result<SourceLines> {
        let lines = read_source_file(&file_name)?
            .split("\n")
            .enumerate()
            .map(|(i, line)| tokenize_line(i + 1, line))
            .collect::<Result<Vec<Line>>>()?;

        Ok(SourceLines {
            lines,
            file_name,
        })
    }
}

fn read_source_file(name: &String) -> Result<String> {
    let mut file = File::open(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_indent_token(line_num: usize, line: &str) -> Result<Token> {
    const INDENT_SIZE: usize = 4;

    let indent = line.chars().take_while(|&c| c == ' ').collect::<String>();
    let indent_spaces = indent.len();
    let error_info = ErrorInfo::new(line_num, 0, indent_spaces);

    if (indent_spaces % INDENT_SIZE) != 0 {
        Err(InvalidIndent(error_info))
    } else {
        Ok(Token::new(
            Indent(indent_spaces / INDENT_SIZE),
            indent,
            error_info
        ))
    }
}

fn tokenize_line(line_num: usize, line: &str) -> Result<Line> {

    let mut tokens = vec![get_indent_token(line_num, line)?];

    let mut lexer = TokenType::lexer(&line);

    while let Some(next_token) = lexer.next() {
        let span = lexer.span();

        let error_info = ErrorInfo::new(line_num, span.start, span.end);
        let token_type = next_token
            .map_err(|_| InvalidToken(error_info))?;
        
        tokens.push(Token::new(
            token_type,
            lexer.slice().to_string(),
            error_info
        ));
    }

    Ok(tokens)
}
