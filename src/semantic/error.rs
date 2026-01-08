use thiserror::Error;
use crate::operators::binary_operators::BinaryOperatorType;
use crate::operators::unary_operators::UnaryOperatorType;
use crate::types::data_type::DataType;

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("Error:")]
    MismatchedUnaryOperatorTypes(UnaryOperatorType, Option<DataType>),
    
    #[error("Error:")]
    MismatchedBinaryOperatorTypes(BinaryOperatorType, Option<DataType>, Option<DataType>),
}

pub type SemanticResult<T> = Result<T, SemanticError>;
