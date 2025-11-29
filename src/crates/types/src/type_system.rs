
#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeCategory {
    Number,
    Immutable,
    Mutable,
    Sequence,
    Callable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
    pub primitive: PrimitiveType,
    pub category: TypeCategory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Keyword,
    Literal(TypeInfo),
    Operator,
    Delimiter,
}

#[derive(Debug)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub lexeme: &'src str,
    pub line: u32,
    pub start: u32,
    pub end: u32,
}