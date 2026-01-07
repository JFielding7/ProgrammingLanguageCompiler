use crate::compiler_context::CompilerContext;
use crate::error::spanned_error::SpannableError;
use crate::lexer::error::LexerError::{InvalidToken, UnalignedIndent};
use crate::lexer::token::TokenType::Indent;
use crate::lexer::token::{Token, TokenType};
use crate::source::source_file::SourceFile;
use crate::source::source_span::SourceSpan;
use logos::Logos;
use crate::error::compiler_error::CompilerResult;
use crate::lexer::error::LexerResult;

struct Line<'a> {
    index: usize,
    content: &'a str,
}

impl<'a> Line<'a> {
    fn new(index: usize, content: &'a str) -> Self {
        Line { index, content }
    }
}

type LineTokens = Vec<Token>;

#[derive(Debug)]
pub struct TokenizedLines(Vec<LineTokens>);

impl IntoIterator for TokenizedLines {
    type Item = LineTokens;
    type IntoIter = std::vec::IntoIter<LineTokens>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct Lexer<'a> {
    source_file: &'a SourceFile,
    ctx: &'a mut CompilerContext,
}

impl<'a> Lexer<'a> {
    fn new(source_file: &'a SourceFile, ctx: &'a mut CompilerContext) -> Self {
        Self { ctx, source_file }
    }

    fn get_indent_token(&mut self, line: &Line) -> LexerResult<Token> {
        const INDENT_SIZE: usize = 4;

        let indent1 = line.content.chars().take_while(|&c| c == ' ').collect::<String>();
        let indent = self.ctx.get_symbol(indent1.as_str());

        let indent_spaces = indent1.len();
        let span = SourceSpan::new(line.index, 0, indent_spaces);

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

    fn tokenize(&mut self, line: &Line) -> LexerResult<LineTokens> {

        let mut tokens = vec![
            self.get_indent_token(line)?
        ];

        let mut lexer = TokenType::lexer(line.content);

        while let Some(next_token) = lexer.next() {
            let span = lexer.span();
            let string = lexer.slice().to_string();

            let source_span = SourceSpan::new(
                line.index, span.start, span.end
            );

            let token_type = next_token
                .map_err(|_| {
                    InvalidToken(string).at(source_span)
                })?;

            tokens.push(Token::new(
                token_type,
                self.ctx.get_symbol(lexer.slice()),
                source_span
            ));
        }

        Ok(tokens)
    }

    pub fn lex_source_file(source_file: &SourceFile, ctx: &mut CompilerContext) -> LexerResult<TokenizedLines> {
        let mut lexer = Lexer::new(source_file, ctx);

        let lines = source_file.into_iter()
            .enumerate()
            .map(|(i, content)| {
                let line = Line::new(i, content);
                lexer.tokenize(&line)
            })
            .collect::<LexerResult<Vec<LineTokens>>>()?;

        Ok(TokenizedLines(lines))
    }
}
