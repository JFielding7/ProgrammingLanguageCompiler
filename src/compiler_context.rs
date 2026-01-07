use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

pub struct CompilerContext {
    string_interner: StringInterner<DefaultBackend>,
}

impl CompilerContext {
    pub fn new() -> Self {
        Self {
            string_interner: StringInterner::default(),
        }
    }

    pub fn get_symbol(&mut self, string: &str) -> DefaultSymbol {
        self.string_interner.get_or_intern(string)
    }

    pub fn resolve_symbol(&mut self, symbol: DefaultSymbol) -> Option<&str> {
        self.string_interner.resolve(symbol)
    }
}
