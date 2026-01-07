use string_interner::DefaultSymbol;

#[derive(Debug)]
pub struct TypeAnnotation {
    type_name: DefaultSymbol,
    inner_types: Option<Vec<TypeAnnotation>>,
}

impl TypeAnnotation {
    pub fn new(type_name: DefaultSymbol) -> Self {
        Self {
            type_name,
            inner_types: None,
        }
    }

    pub fn with_params(type_name: DefaultSymbol, inner_types: Vec<TypeAnnotation>) -> Self {
        Self {
            type_name,
            inner_types: Some(inner_types),
        }
    }

}
