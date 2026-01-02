use std::mem::discriminant;
use logos::Logos;
use crate::error_util::SourceLocation;
use TokenType::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_str: String,
    pub location: SourceLocation,
}

#[derive(Logos, Debug, Clone)]
pub enum TokenType {
    #[token("fn")]
    Fn,

    #[token("if")]
    If,
    #[token("elif")]
    Elif,
    #[token("else")]
    Else,

    #[token("for")]
    For,
    #[token("while")]
    While,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,

    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,

    #[token("=")]
    Equals,
    #[token("+=")]
    PlusEquals,
    #[token("-=")]
    MinusEquals,
    #[token("*=")]
    StarEquals,
    #[token("/=")]
    SlashEquals,
    #[token("%=")]
    PercentEquals,

    #[token("<<=")]
    DoubleLeftArrowEquals,
    #[token(">>=")]
    DoubleRightArrowEquals,
    #[token("&=")]
    AmpersandEquals,
    #[token("|=")]
    PipeEquals,
    #[token("^=")]
    CaretEquals,

    #[token("==")]
    DoubleEquals,
    #[token("!=")]
    ExclamationEquals,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEquals,
    #[token(">=")]
    GreaterEquals,

    #[token("&&")]
    DoubleAmpersand,
    #[token("||")]
    DoublePipe,
    #[token("!")]
    Exclamation,

    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,
    #[token("^")]
    Caret,
    #[token("~")]
    Tilde,
    #[token("<<")]
    DoubleLeftArrow,
    #[token(">>")]
    DoubleRightArrow,

    #[token(".")]
    Dot,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,

    #[token(",")]
    Comma,

    #[regex(r"[0-9]+")]
    IntLiteral,
    #[regex(r#""[^"]*""#)]
    StringLiteral,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"[ \t\f\v]+", logos::skip)]
    Whitespace,
    #[regex(r"//.*", logos::skip, allow_greedy = true)]
    Comment,

    Indent(usize),
}

impl Token {
    pub fn new(token_type: TokenType, token_str: String, location: SourceLocation) -> Self {
        Self { token_type, token_str, location }
    }

    pub fn is_legal_statement_boundary(&self) -> bool {
        match self.token_type {
            Plus |
            Minus |
            Equals |
            OpenParen |
            Comma => false,
            _ => true,
        }
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        discriminant(&self.token_type) == discriminant(other)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.token_str.as_str())
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Identifier => "Identifier",
            OpenParen => "'('",
            CloseParen => "')'",
            Comma => "','",
            Indent(_) => "Indent",
            _ => ""
        })
    }
}
