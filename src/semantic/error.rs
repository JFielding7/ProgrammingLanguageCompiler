use thiserror::Error;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::types::data_type::DataType;

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("Error:")]
    MismatchedUnaryOperatorTypes(UnaryOperator, Option<DataType>),
    
    #[error("Error:")]
    MismatchedBinaryOperatorTypes(BinaryOperator, Option<DataType>, Option<DataType>),
}

pub type SemanticResult<T> = Result<T, SemanticError>;
