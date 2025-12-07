use lexer::tokenizer::{Token, std_ids};

pub fn t(value: &str, id: u32) -> Token {
    Token {
        value: value.to_string(),
        token_type: "TestType".to_string(),
        std_token_id: id,
        lexem: value.to_string(),
    }
}

pub const ID_VAR: u32 = std_ids::LET;       // 53
pub const ID_FUNC: u32 = std_ids::FUNCTION; // 46
pub const ID_STRUCT: u32 = std_ids::STRUCT; // 48
pub const ID_NAME: u32 = std_ids::IDENTIFIER; // 70
pub const ID_INT: u32 = std_ids::INT_TYPE;    // 71
pub const ID_EQ: u32 = std_ids::ASSIGN;       // 33
pub const ID_L_BRACE: u32 = std_ids::L_BRACE; // 1
pub const ID_R_BRACE: u32 = std_ids::R_BRACE; // 2
pub const ID_L_PAREN: u32 = std_ids::L_PAREN; // 3
pub const ID_R_PAREN: u32 = std_ids::R_PAREN; // 4
pub const ID_COLON: u32 = std_ids::COLON;     // 7
pub const ID_COMMA: u32 = std_ids::COMMA;     // 9
pub const ID_NUM: u32 = std_ids::INT_LITERAL; // 71 (або 72 для float)