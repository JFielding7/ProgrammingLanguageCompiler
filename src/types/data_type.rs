use string_interner::DefaultSymbol;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum DataType {
    Unit,
    Bool,
    Int,
    String,
    UserDefined(DefaultSymbol),
    // TODO: generics
}
