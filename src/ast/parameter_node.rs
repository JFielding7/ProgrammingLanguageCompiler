#[derive(Debug)]
pub struct ParameterNode {
    name: String,
    data_type: String,
}

impl ParameterNode {
    pub(crate) fn new(name: String, data_type: String) -> Self {
        Self { name, data_type }
    }
}