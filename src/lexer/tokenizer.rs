use crate::source::source_span::SourceSpan;
use crate::lexer::error::LexerErrorType::{InvalidToken, UnalignedIndent};
use crate::lexer::error::LexerResult;
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use crate::error::spanned_error::WithSpan;
use logos::Logos;

type Line = Vec<Token>;

pub struct SourceLines {
    lines: Vec<Line>,
}

impl IntoIterator for SourceLines {
    type Item = Line;
    type IntoIter = std::vec::IntoIter<Line>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

fn get_indent_token(line_index: usize, line: &str) -> LexerResult<Token> {
    const INDENT_SIZE: usize = 4;

    let indent = line.chars().take_while(|&c| c == ' ').collect::<String>();
    let indent_spaces = indent.len();
    let location = SourceSpan::new(line_index, 0, indent_spaces);

    if (indent_spaces % INDENT_SIZE) != 0 {
        Err(UnalignedIndent(indent_spaces).at(location))
    } else {
        Ok(Token::new(
            Indent(indent_spaces / INDENT_SIZE),
            indent,
            location
        ))
    }
}

fn tokenize_line(line_index: usize, line: &str) -> LexerResult<Line> {
    
    let mut tokens = vec![
        get_indent_token(line_index, line)?
    ];

    let mut lexer = TokenType::lexer(line);

    while let Some(next_token) = lexer.next() {
        let span = lexer.span();
        let string = lexer.slice().to_string();

        let location = SourceSpan::new(
            line_index, span.start, span.end
        );

        let token_type = next_token
            .map_err(|_| {
                InvalidToken(string).at(location.clone())
            })?;
        
        tokens.push(Token::new(
            token_type,
            lexer.slice().to_string(),
            location
        ));
    }

    Ok(tokens)
}

pub fn lex(src: String) -> LexerResult<SourceLines> {

    let lines = src.lines()
        .enumerate()
        .map(|(i, line)| tokenize_line(i, line))
        .collect::<LexerResult<Vec<Line>>>()?;

    Ok(SourceLines {
        lines,
    })
}
