use crate::compiler_context::scope::{Scope, ScopeId};
use crate::compiler_context::symbol::Symbol;
use string_interner::DefaultSymbol;

pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::global()],
        }
    }
    
    pub fn global_scope(&self) -> ScopeId {
        ScopeId::new(0)
    }
    
    pub fn add_scope(&mut self, scope: Scope) -> ScopeId {
        let id = self.scopes.len();
        self.scopes.push(scope);
        ScopeId::new(id)
    }

    pub fn lookup(&self, name: DefaultSymbol, scope: ScopeId) -> Option<&Symbol> {
        
        let mut curr_scope = Some(scope);
        
        while let Some(id) = curr_scope {
            
            let scope = &self.scopes[id.as_usize()];
            
            if let Some(symbol) = scope.lookup(name) {
                return Some(symbol);
            }
            
            curr_scope = scope.parent;
        }
        
        None
    }
}


