use std::hash::Hash;
use crate::operators::binary_operators::BinaryOperatorType;
use crate::operators::unary_operators::UnaryOperatorType;
use crate::types::data_type::DataType;
use crate::types::data_type::DataType::Bool;

pub trait BuiltinTypeResolver<Operands> {
    fn resolve_builtins(&self, operand: &Operands) -> Option<DataType>;
}

impl BuiltinTypeResolver<DataType> for UnaryOperatorType {
    fn resolve_builtins(&self, operand: &DataType) -> Option<DataType> {
        use UnaryOperatorType::*;
        use DataType::*;

        Some(match self {
            Neg => match operand {
                Int => Int,
                _ => return None,
            },

            Not => match operand {
                Bool => Bool,
                _ => return None,
            },

            BitNot => match operand {
                Int => Int,
                _ => return None,
            },

            PreInc | PreDec | PostInc | PostDec => match operand {
                Int => Int,
                _ => return None,
            },
        })
    }
}


impl BuiltinTypeResolver<(DataType, DataType)> for BinaryOperatorType {
    fn resolve_builtins(&self, operands: &(DataType, DataType)) -> Option<DataType> {
        use BinaryOperatorType::*;
        use DataType::*;

        let (lhs_type, rhs_type) = operands.clone();

        Some(match self {
            Assign => {
                if lhs_type == rhs_type {
                    rhs_type
                } else {
                    return None
                }
            },

            AddAssign | SubAssign | MulAssign | DivAssign | ModAssign => match (lhs_type, rhs_type) {
                (Int, Int) => Int,
                _ => return None,
            },

            LeftShiftAssign | RightShiftAssign => match (lhs_type, rhs_type) {
                (Int, Int) => Int,
                _ => return None,
            },

            AndAssign | XorAssign | OrAssign => match (lhs_type, rhs_type) {
                (Int, Int) => Int,
                _ => return None,
            },

            Add => match (lhs_type, rhs_type) {
                (Int, Int) => Int,
                (String, String) => String,
                _ => return None,
            },

            Sub | Mul | Div | Mod => match (lhs_type, rhs_type) {
                (Int, Int) => Int,
                _ => return None,
            },

            BitAnd | BitOr | BitXor | LeftShift | RightShift => match (lhs_type, rhs_type) {
                (Int, Int) => Int,
                _ => return None,
            },

            Equal | NotEquals => {
                if lhs_type == rhs_type {
                    Bool
                } else {
                    return None
                }
            },

            LessThan | LessOrEqual | GreaterThan | GreaterOrEqual => match (lhs_type, rhs_type) {
                (Int, Int) | (String, String) => Bool,
                _ => return None,
            },

            LogicalAnd | LogicalOr => match (lhs_type, rhs_type) {
                (Bool, Bool) => Bool,
                _ => return None,
            },

            CommaOperator => rhs_type,
        })
    }
}
