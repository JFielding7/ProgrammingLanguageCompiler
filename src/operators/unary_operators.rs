use strum::EnumCount;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumCount)]
pub enum UnaryOperatorType {
    Neg,
    Not,
    BitNot,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}
