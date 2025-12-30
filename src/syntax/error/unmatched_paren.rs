use thiserror::Error;
use crate::error_util::ErrorLocation;
use crate::lexer::token::TokenType;

#[derive(Debug, Error)]
pub struct UnmatchedParenError {
    paren_type: TokenType,
    error_location: ErrorLocation,
}

impl UnmatchedParenError {
    pub fn new(
        paren_type: TokenType,
        error_location: ErrorLocation,
    ) -> Self {
        Self {
            paren_type,
            error_location,
        }
    }
}

impl std::fmt::Display for UnmatchedParenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unmatched {}\n{}", self.paren_type, self.error_location)
    }
}
