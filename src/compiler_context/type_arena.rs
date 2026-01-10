use strum::IntoEnumIterator;
use crate::types::data_type::{BuiltinType, DataType};


pub struct TypeArena {
    data_types: Vec<DataType>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DataTypeId(usize);

impl TypeArena {
    pub fn new() -> Self {
        Self {
            data_types: BuiltinType::iter()
                .map(|t| DataType::Builtin(t))
                .collect(),
        }
    }

    pub fn get(&self, id: DataTypeId) -> &DataType {
        &self.data_types[id.0]
    }

    pub fn builtin_type_id(&self, builtin_type: BuiltinType) -> DataTypeId {
        DataTypeId(builtin_type as usize)
    }
}
