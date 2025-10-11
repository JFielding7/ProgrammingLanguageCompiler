use std::io;

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error("Error: No input files")]
    NoInputFiles,
    #[error("Error: Line {0}: Invalid Token")]
    InvalidToken(usize),
    #[error("Error: Line {0}: Expected token")]
    ExpectedToken(usize),
    #[error("Error: Line {0}: Invalid Indent Size")]
    InvalidIndent(usize),
    #[error("{0}")]
    IOError(#[from] io::Error)
}

pub fn raise_compiler_error(error: CompilerError) {
    println!("{error}");
    std::process::exit(1);
}

pub type Result<T> = std::result::Result<T, CompilerError>;
