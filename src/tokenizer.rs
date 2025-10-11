use crate::compiler_error::CompilerError::{InvalidIndent, InvalidToken};
use crate::compiler_error::Result;
use crate::tokenizer::TokenType::Indent;
use logos::Logos;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct SourceFileTokens {
    file_name: String,
    pub(crate) lines: Vec<Vec<Token>>
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    token_str: String,
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
    Indent(usize),

    #[regex(r"[ \t\f]+", logos::skip)]
    Whitespace,
    #[regex(r"//.*", logos::skip)]
    Comment,
}

impl Token {
    fn new(token_type: TokenType, token_str: String) -> Self {
        Self {
            token_type,
            token_str
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

    if (indent_spaces & 3) != 0 {
        return Err(InvalidIndent(line_num));
    }

    let mut tokens = vec![Token::new(Indent(indent_spaces >> 2), "".to_string())];

    let mut lexer = TokenType::lexer(&line);

    while let Some(next_token) = lexer.next() {
        let token_type = next_token.map_err(|_| InvalidToken(line_num))?;
        tokens.push(Token::new(token_type, lexer.slice().to_string()));
    }

    Ok(tokens)
}

pub fn tokenize_file(filename: String) -> Result<SourceFileTokens> {
    Ok(SourceFileTokens {
        lines: read_source_file(&filename)?
            .split("\n")
            .enumerate()
            .map(|(i, line)| tokenize_line(i + 1, line))
            .collect::<Result<Vec<Vec<Token>>>>()?,
        file_name: filename,
    })
}
