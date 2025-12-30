use logos::Logos;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use crate::error_util::ErrorLocation;
use crate::lexer::error::invalid_token::InvalidTokenError;
use crate::lexer::error::unaligned_indent::UnalignedIndentError;
use crate::lexer::error::{LexerError, LexerResult};
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};

type Line = Vec<Token>;
pub struct SourceLines(Vec<Line>);


impl SourceLines {

    pub fn lex(path: &Path, src: String) -> LexerResult<Self> {

        let lines = src.lines()
            .enumerate()
            .map(|(i, line)| tokenize_line(path, i + 1, Rc::new(line.to_string())))
            .collect::<LexerResult<Vec<Line>>>()?;

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

fn get_indent_token(path_buf: Rc<PathBuf>, line_content: Rc<String>, line_num: usize) -> LexerResult<Token> {
    const INDENT_SIZE: usize = 4;

    let indent = line_content.chars().take_while(|&c| c == ' ').collect::<String>();
    let indent_spaces = indent.len();
    let error_location = ErrorLocation::new(path_buf, line_content, line_num, 0, indent_spaces);

    if (indent_spaces % INDENT_SIZE) != 0 {
        Err(UnalignedIndentError::new(indent_spaces, error_location).into())
    } else {
        Ok(Token::new(
            Indent(indent_spaces / INDENT_SIZE),
            indent,
            error_location
        ))
    }
}

fn tokenize_line(path: &Path, line_num: usize, line_content: Rc<String>) -> LexerResult<Line> {

    let path_buf = Rc::new(path.to_path_buf());

    let mut tokens = vec![
        get_indent_token(path_buf.clone(), line_content.clone(), line_num)?
    ];

    let mut lexer = TokenType::lexer(&line_content);

    while let Some(next_token) = lexer.next() {
        let span = lexer.span();
        let string = lexer.slice().to_string();

        let error_location = ErrorLocation::new(
            path_buf.clone(), line_content.clone(), line_num, span.start, span.end
        );

        let token_type = next_token
            .map_err(|_| {
                let e: LexerError = InvalidTokenError::new(string, error_location.clone()).into();
                e
            })?;
        
        tokens.push(Token::new(
            token_type,
            lexer.slice().to_string(),
            error_location
        ));
    }

    Ok(tokens)
}
