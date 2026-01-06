use crate::ast::ast_arena::ASTNodeId;

#[derive(Debug)]
pub struct BinaryOperatorNode {
    op_type: BinaryOperatorType,
    left: ASTNodeId,
    right: ASTNodeId,
}

impl BinaryOperatorNode {
    pub fn new(
        op_type: BinaryOperatorType,
        left: ASTNodeId,
        right: ASTNodeId,
    ) -> Self {
        Self {
            op_type,
            left,
            right,
        }
    }
}

#[derive(Debug)]
pub enum BinaryOperatorType {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    LeftShiftAssign,
    RightShiftAssign,
    AndAssign,
    XorAssign,
    OrAssign,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    BitAnd,
    BitOr,
    BitXor,

    LeftShift,
    RightShift,

    Equal,
    NotEquals,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,

    LogicalAnd,
    LogicalOr,

    CommaOperator,
}
