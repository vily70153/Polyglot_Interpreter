use types::type_system;


#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    Keyword,
    Literal(type_system::TypeInfo),
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
