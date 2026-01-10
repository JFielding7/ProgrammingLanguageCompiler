use std::collections::HashMap;
use string_interner::DefaultSymbol;
use crate::compiler_context::symbol::Symbol;

pub struct Scope {
    symbols: HashMap<DefaultSymbol, Symbol>,
    pub(crate) parent: Option<ScopeId>,
}

impl Scope {
    pub fn global() -> Self {
        Self {
            parent: None,
            symbols: HashMap::new(),
        }
    }

    pub fn with_parent(parent: ScopeId) -> Self {
        Self {
            parent: Some(parent),
            symbols: HashMap::new()
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name, symbol);
    }

    pub fn lookup(&self, name: DefaultSymbol) -> Option<&Symbol> {
        match self.symbols.get(&name) {
            Some(symbol) => Some(symbol),
            None => None
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ScopeId(usize);

impl ScopeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}