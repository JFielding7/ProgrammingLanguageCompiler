use thiserror::Error;
use crate::error_util::ErrorLocation;

#[derive(Debug, Error)]
pub struct UnalignedIndentError {
    indent_size: usize,
    error_location: ErrorLocation,
}

impl UnalignedIndentError {
    pub fn new(indent_size: usize, error_location: ErrorLocation) -> Self {
        Self {
            indent_size,
            error_location,
        }
    }
}

impl std::fmt::Display for UnalignedIndentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unaligned Indent: Indent size {} is not a multiple of 4\n{}",
               self.indent_size,
               self.error_location
        )
    }
}
