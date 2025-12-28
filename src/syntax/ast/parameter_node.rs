#[derive(Debug)]
pub struct ParameterNode {
    name: String,
    data_type: String,
}

impl ParameterNode {
    pub fn new(name: String, data_type: String) -> Self {
        Self { name, data_type }
    }
}