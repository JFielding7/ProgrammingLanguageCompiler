use crate::source::source_file::SourceFile;
use crate::source::source_span::SourceSpan;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{error_type}")]
pub struct SpannedError {
    pub error_type: Box<dyn SpannableError>,
    pub span: SourceSpan,
}

impl SpannedError {
    fn new(error_type: Box<dyn SpannableError>, span: SourceSpan) -> Self {
        Self {
            error_type, span
        }
    }

    pub fn format(&self, source_file: &SourceFile) -> String {
        format!("{}\n{}", self, self.span.format_source_span(source_file))
    }
}

pub trait SpannableError: std::error::Error where Self: 'static {
    fn at(self, span: SourceSpan) -> SpannedError
    where Self: Sized {
        SpannedError::new(Box::new(self), span)
    }
}
