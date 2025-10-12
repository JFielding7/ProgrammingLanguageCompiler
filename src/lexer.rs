use crate::compiler_error::CompilerError::{ExpectTokenNotFound, InvalidToken};
use crate::compiler_error::{ErrorInfo, Result};
use crate::lexer::TokenType::Indent;
use logos::Logos;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::mem::discriminant;

#[derive(Debug)]
pub struct SourceFileTokens {
    pub file_name: String,
    pub lines: Vec<Vec<Token>>
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token_str: String,
    pub error_info: ErrorInfo,
}

#[derive(Logos, Debug, Clone)]
pub enum TokenType {
    #[token("fn")]
    Fn,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("=")]
    Assign,

    #[regex(r"[0-9]+")]
    IntLiteral,
    #[regex(r#""[^"]*""#)]
    StringLiteral,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token(",")]
    Comma,

    #[regex(r"[ \t\f\v]+", logos::skip)]
    Whitespace,
    #[regex(r"//.*", logos::skip)]
    Comment,

    Indent(usize),
}

impl Token {
    fn new(token_type: TokenType, token_str: String, error_info: ErrorInfo) -> Self {
        Self { token_type, token_str, error_info }
    }

    pub fn is_legal_statement_boundary(&self) -> bool {
        use TokenType::*;

        match self.token_type {
            Plus |
            Minus |
            Assign |
            OpenParen |
            Comma => false,
            _ => true,
        }
    }
}

pub trait CheckTokenType {
    fn check_type(self, token_type: TokenType) -> Result<Token>;
}

impl CheckTokenType for Option<Token> {
    fn check_type(self, token_type: TokenType) -> Result<Token> {
        match self {
            None => Err(ExpectTokenNotFound(None, token_type)),
            Some(token) => {
                if discriminant(&token.token_type) == discriminant(&token_type) {
                    Ok(token)
                } else {
                    Err(ExpectTokenNotFound(Some(token), token_type))
                }
            }
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;

        // match self {
        //     Int => "int",
        //     Fn => "fn",
        //     Plus => "+",
        //     Minus => "-",
        //     Assign => "="
        //     IntLiteral => {}
        //     StringLiteral => {}
        //     Identifier => {}
        //     OpenParen => {}
        //     CloseParen => {}
        //     Comma => {}
        //     Whitespace => {}
        //     Comment => {}
        //     Indent(_) => {}
        // }
        write!(f, "")
    }
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
