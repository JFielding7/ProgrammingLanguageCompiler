use thiserror::Error;
use crate::error::compiler_error::CompilerError::{NoInputFiles, FileRead, Spanned};
use crate::error::spanned_error::{SpannableError, SpannedError};
use crate::lexer::error::LexerError;
use crate::semantic::error::SemanticError;
use crate::source::source_file::SourceFile;
use crate::syntax::error::SyntaxError;

#[derive(Error, Debug)]
pub enum CompilerError {
    NoInputFiles,

    FileRead {
        file_name: String,
        #[source]
        error: std::io::Error,
    },

    Spanned(SourceFile, #[source] SpannedError),
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoInputFiles => write!(f, "Error: No Input Files"),
            FileRead { file_name, error } => {
                write!(f, "Error: {file_name}: {error}")
            }
            Spanned(file, e) => write!(f, "{}", e.format(file)),
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
    LexerError,
    SyntaxError,
    SemanticError
}

pub type CompilerResult = Result<(), CompilerError>;
