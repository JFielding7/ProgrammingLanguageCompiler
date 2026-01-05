use crate::source::source_span::SourceSpan;
use crate::lexer::error::LexerErrorType::{InvalidToken, UnalignedIndent};
use crate::lexer::error::LexerResult;
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use crate::error::spanned_error::WithSpan;
use logos::Logos;
use crate::source::source_file::SourceFile;

type LineTokens = Vec<Token>;

pub struct TokenizedLines {
    lines: Vec<LineTokens>,
}

impl IntoIterator for TokenizedLines {
    type Item = LineTokens;
    type IntoIter = std::vec::IntoIter<LineTokens>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

struct Line<'a> {
    index: usize,
    content: &'a str,
}

impl<'a> Line<'a> {
    fn new(index: usize, content: &'a str) -> Self {
        Line { index, content }
    }

    fn get_indent_token(&self) -> LexerResult<Token> {
        const INDENT_SIZE: usize = 4;

        let indent = self.content.chars().take_while(|&c| c == ' ').collect::<String>();
        let indent_spaces = indent.len();
        let span = SourceSpan::new(self.index, 0, indent_spaces);

        if (indent_spaces % INDENT_SIZE) != 0 {
            Err(UnalignedIndent(indent_spaces).at(span))
        } else {
            Ok(Token::new(
                Indent(indent_spaces / INDENT_SIZE),
                indent,
                span
            ))
        }
    }

    fn tokenize(&self) -> LexerResult<LineTokens> {

        let mut tokens = vec![
            self.get_indent_token()?
        ];

        let mut lexer = TokenType::lexer(self.content);

        while let Some(next_token) = lexer.next() {
            let span = lexer.span();
            let string = lexer.slice().to_string();

            let source_span = SourceSpan::new(
                self.index, span.start, span.end
            );

            let token_type = next_token
                .map_err(|_| {
                    InvalidToken(string).at(source_span.clone())
                })?;

            tokens.push(Token::new(
                token_type,
                lexer.slice().to_string(),
                source_span
            ));
        }

        Ok(tokens)
    }
}

pub fn lex(source_code: &SourceFile) -> LexerResult<TokenizedLines> {

    let lines = source_code.into_iter()
        .enumerate()
        .map(|(i, content)| Line::new(i, content).tokenize())
        .collect::<LexerResult<Vec<LineTokens>>>()?;

    Ok(TokenizedLines {
        lines,
    })
}
