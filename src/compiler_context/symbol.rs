use string_interner::DefaultSymbol;
use crate::compiler_context::type_arena::DataTypeId;
use crate::source::source_span::SourceSpan;

pub struct Symbol {
    pub name: DefaultSymbol,
    pub data_type: DataTypeId,
    def_span: SourceSpan,
}
