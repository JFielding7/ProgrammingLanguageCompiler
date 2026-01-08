use crate::semantic::type_resolution::builtin_type_resolver::BuiltinTypeResolver;
use std::collections::HashMap;
use std::hash::Hash;
use crate::types::data_type::DataType;

pub struct OperatorRegistry<OpType: Eq + Hash + BuiltinTypeResolver<Operands>, Operands: Eq + Hash> {
    implementations: HashMap<OpType, HashMap<Operands, DataType>>
}

impl<OpType: Eq + Hash + BuiltinTypeResolver<Operands>, Operands: Eq + Hash> OperatorRegistry<OpType, Operands> {
    
    pub fn new() -> Self {
        Self {
            implementations: HashMap::new()
        }
    }

    pub fn resolve_operation_type(&self, op_type: OpType, operands: &Operands) -> Option<DataType> {
        if let Some(data_type) = op_type.resolve_builtins(operands) {
            return Some(data_type);
        }
        
        let definitions = match self.implementations.get(&op_type) {
            Some(definitions) => definitions,
            None => return None,
        };

        match definitions.get(operands) {
            Some(data_type) => Some(data_type.clone()),
            None => None,
        }
    }
}
