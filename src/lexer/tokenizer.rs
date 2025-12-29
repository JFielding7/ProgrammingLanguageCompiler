use crate::error::compiler_error::CompilerError::{InvalidIndent, InvalidToken};
use crate::error::compiler_error::{CompilerError, Result};
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use logos::Logos;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use crate::error::error_info::ErrorInfo;

type Line = Vec<Token>;
pub struct SourceLines(Vec<Line>);


impl TryFrom<&Path> for SourceLines {
    type Error = CompilerError;

    fn try_from(path: &Path) -> Result<Self> {
        let src = read_to_string(path)?;
        let lines = src.lines()
            .enumerate()
            .map(|(i, line)| tokenize_line(path, i + 1, Rc::new(line.to_string())))
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

fn get_indent_token(path_buf: Rc<PathBuf>, line_content: Rc<String>, line_num: usize) -> Result<Token> {
    const INDENT_SIZE: usize = 4;

    let indent = line_content.chars().take_while(|&c| c == ' ').collect::<String>();
    let indent_spaces = indent.len();
    let error_info = ErrorInfo::new(path_buf, line_content, line_num, 0, indent_spaces);

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

fn tokenize_line(path: &Path, line_num: usize, line_content: Rc<String>) -> Result<Line> {

    let path_buf = Rc::new(path.to_path_buf());

    let mut tokens = vec![
        get_indent_token(path_buf.clone(), line_content.clone(), line_num)?
    ];

    let mut lexer = TokenType::lexer(&line_content);

    while let Some(next_token) = lexer.next() {
        let span = lexer.span();

        let error_info = ErrorInfo::new(
            path_buf.clone(), line_content.clone(), line_num, span.start, span.end
        );
        let token_type = next_token
            .map_err(|_| InvalidToken(error_info.clone()))?;
        
        tokens.push(Token::new(
            token_type,
            lexer.slice().to_string(),
            error_info
        ));
    }

    Ok(tokens)
}
