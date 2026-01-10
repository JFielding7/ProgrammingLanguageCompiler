use strum::EnumIter;
use string_interner::DefaultSymbol;

#[derive(Debug)]
pub enum DataType {
    Builtin(BuiltinType),
    UserDefined(DefaultSymbol),
    // TODO: generics
}

#[derive(Debug, PartialEq, EnumIter)]
pub enum BuiltinType {
    Unit = 0,
    Bool,
    Int,
    String,
}
