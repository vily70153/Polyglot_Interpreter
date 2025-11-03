
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
pub enum TypeCategory {
    Number,
    Immutable,
    Mutable,
    Sequence,
    Callable,
}

#[derive(Debug)]
pub struct TypeInfo {
    pub primitive: PrimitiveType,
    pub category: TypeCategory,
}
