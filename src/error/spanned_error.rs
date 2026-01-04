use crate::error::compiler_error::CompilerError;
use crate::source::source_file::SourceFile;
use crate::source::source_span::SourceSpan;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{error_type}")]
pub struct SpannedError<T: SpannableError> {
    pub error_type: T,
    pub span: SourceSpan,
}

impl<T: SpannableError> SpannedError<T> {
    pub fn new(error_type: T, span: SourceSpan) -> Self {
        Self {
            error_type, span
        }
    }
    
    pub fn compiler_error(self, file: &SourceFile) -> CompilerError {
        CompilerError::Spanned {
            message: format!("{}\n{}", self, self.span.format_source_location(file))
        }
    }
}

pub trait SpannableError: std::error::Error {}

pub trait WithSpan: SpannableError {
    fn at(self, span: SourceSpan) -> SpannedError<Self> 
    where Self: Sized {
        SpannedError::new(self, span)
    }
}

impl<T: SpannableError> WithSpan for T {}
