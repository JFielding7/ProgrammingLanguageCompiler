struct SymbolTableId(usize);

pub struct SymbolTable {
    parent_id: Option<SymbolTableId>,
}