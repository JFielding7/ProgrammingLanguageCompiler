use crate::compiler_error::CompilerError::InvalidToken;
use crate::compiler_error::Result;
use crate::lexer::TokenType::{Comma, Indent, Minus, OpenParen};
use logos::Logos;
use std::fs::File;
use std::io::Read;


#[derive(Debug)]
pub struct SourceFileTokens {
    pub file_name: String,
    pub lines: Vec<Vec<Token>>
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    token_str: Option<String>,
    line_num: usize,
}

#[derive(Logos, Debug)]
enum TokenType {
    #[token("int")]
    Int,
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
    fn new(token_type: TokenType, token_str: Option<String>, line_num: usize) -> Self {
        Self {
            token_type,
            token_str,
            line_num
        }
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

fn read_source_file(name: &String) -> Result<String> {
    let mut file = File::open(name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn tokenize_line(line_num: usize, line: &str) -> Result<Vec<Token>> {
    let indent_spaces = line.chars().take_while(|&c| c == ' ').count();

    let mut tokens = vec![Token::new(Indent(indent_spaces), None, line_num)];

    let mut lexer = TokenType::lexer(&line);

    while let Some(next_token) = lexer.next() {
        let token_type = next_token.map_err(|_| InvalidToken(line_num))?;

        tokens.push(Token::new(
            token_type,
            Some(lexer.slice().to_string()),
            line_num
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
