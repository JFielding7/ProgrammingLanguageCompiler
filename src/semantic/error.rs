mod use_before_def;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SemanticErrorType {

    #[error("Error: {0}")]
    UseBeforeDef(#[from] UseBeforeDefError),
}