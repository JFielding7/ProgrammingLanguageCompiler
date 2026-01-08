use strum::EnumCount;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumCount)]
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
