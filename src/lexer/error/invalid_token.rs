use thiserror::Error;
use crate::error_util::ErrorLocation;

#[derive(Debug, Error)]
pub struct InvalidTokenError {
    invalid_token_string: String,
    error_location: ErrorLocation,
}

impl InvalidTokenError {
    pub fn new(invalid_token_string: String, error_location: ErrorLocation) -> Self {
        Self {
            invalid_token_string,
            error_location,
        }
    }
}

impl std::fmt::Display for InvalidTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized token: {}\n{}",
               self.invalid_token_string,
               self.error_location
        )
    }
}
