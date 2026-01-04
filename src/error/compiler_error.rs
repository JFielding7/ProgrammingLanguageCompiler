use thiserror::Error;

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

    #[error("Error: {message}")]
    Spanned {
        message: String
    },
}

pub type CompilerResult = Result<(), CompilerError>;
