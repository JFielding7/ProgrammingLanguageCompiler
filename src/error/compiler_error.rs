use thiserror::Error;
use crate::error::compiler_error::CompilerError::{Lexer, Syntax};
use crate::error::spanned_error::SpannableError;
use crate::lexer::error::{LexerError, LexerErrorType};
use crate::source::source_file::SourceFile;
use crate::syntax::error::{SyntaxError, SyntaxErrorType};

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Error: No Input Files")]
    NoInputFiles,

    #[error("Error: {file_name}: {error}")]
    FileRead {
        file_name: String,
        #[source]
        error: std::io::Error,
    },

    #[error(transparent)]
    Lexer(#[from] LexerError),

    #[error(transparent)]
    Syntax(#[from] SyntaxError),
}

impl CompilerError {
    pub fn format(&self, curr_source_file: Option<SourceFile>) -> String {
        match (self, curr_source_file) {
            (Lexer(e), Some(src)) => e.format(src),
            (Syntax(e), Some(src)) => e.format(src),
            _ => self.to_string(),
        }
    }
}

macro_rules! impl_spannable_errors {
    ($($error_type:ident),*) => {
        $(
            impl SpannableError for $error_type {}
        )*
    };
}

impl_spannable_errors! {
    LexerErrorType,
    SyntaxErrorType
}

pub type CompilerResult = Result<(), CompilerError>;
