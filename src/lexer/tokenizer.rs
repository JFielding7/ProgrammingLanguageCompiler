use crate::error::compiler_error::CompilerError::{InvalidIndent, InvalidToken};
use crate::error::compiler_error::{CompilerError, Result};
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use logos::Logos;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::Path;
use crate::error::error_info::ErrorInfo;

type Line = Vec<Token>;
pub struct SourceLines(Vec<Line>);


impl TryFrom<&Path> for SourceLines {
    type Error = CompilerError;

    fn try_from(path: &Path) -> Result<Self> {
        let src = read_to_string(path)?;
        let lines = src
            .lines()
            .enumerate()
            .map(|(i, line)| tokenize_line(i + 1, line))
            .collect::<Result<Vec<Line>>>()?;

        Ok(Self(lines))
    }
}

impl IntoIterator for SourceLines {
    type Item = Line;
    type IntoIter = std::vec::IntoIter<Line>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
