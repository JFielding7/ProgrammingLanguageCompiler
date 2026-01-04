use crate::lexer::token::TokenType;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct UnmatchedParenError {
    paren_type: TokenType,
}

impl UnmatchedParenError {
    pub fn new(
        paren_type: TokenType,
    ) -> Self {
        Self {
            paren_type,
        }
    }
}

impl std::fmt::Display for UnmatchedParenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unmatched {}", self.paren_type)
    }
}
