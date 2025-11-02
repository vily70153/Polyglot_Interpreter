
#[derive(Debug, Clone, Copy)]
pub enum PrimitiveType {
    Int,
    Float,
    String,
    Tuple,
    List,
    Dictionary,
    Function,
    Class,
    Struct,
}

#[derive(Debug, Clone, Copy)]
enum TypeCategory {
    Number,
    Immutable,
    Mutable,
    Sequence,
    Callable,
}

#[derive(Debug)]
struct TypeInfo {
    primitive: PrimitiveType,
    category: TypeCategory,
}

#[derive(Debug)]
struct Token<'src> {
    pub start: u32,
    pub current: u32,
    pub line: u32,
    pub type_info: TypeInfo,
    pub token_str: &'src str,
}
