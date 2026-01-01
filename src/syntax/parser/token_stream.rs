use crate::lexer::token::Token;
use crate::syntax::parser::statement::Statement;
use std::iter::Peekable;
use std::slice::Iter;
use crate::error_util::SourceLocation;

pub struct TokenStream<'a> {
    iter: Peekable<Iter<'a, Token>>,
    pub(crate) prev: &'a Token,
}


impl<'a> TokenStream<'a> {
    pub fn from_statement_suffix(statement: &'a Statement, start: usize) -> Self {
        let prev = &statement[start - 1];
        let iter = statement[start..].iter().peekable();

        Self {
            iter,
            prev
        }
    }

    pub fn peek(&mut self) -> Option<&&'a Token> {
        self.iter.peek()
    }

    pub fn empty(&mut self) -> bool{
        self.iter.peek().is_none()
    }
    
    pub fn prev_location(&self) -> SourceLocation {
        self.prev.location.clone()
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.iter.next();

        if let Some(t) = token {
            self.prev = t;
        }

        token
    }
}
