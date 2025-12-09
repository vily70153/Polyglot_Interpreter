use db::db::DB;
use tracing::info;
use std::iter::Peekable;
use std::str::Chars;

pub mod std_ids {
    pub const LET: u32 = 53;
    pub const FUNCTION: u32 = 46;  // function / функція
    pub const IDENTIFIER: u32 = 70;
    pub const INT_LITERAL: u32 = 71;
    pub const FLOAT_LITERAL: u32 = 72;
    pub const STRING_LITERAL: u32 = 73;
    
    pub const ASSIGN: u32 = 33;    // =
    pub const PLUS: u32 = 13;      // +
    pub const MINUS: u32 = 14;     // -
    pub const MULTIPLY: u32 = 15;  // *
    pub const DIVIDE: u32 = 16;    // /
    
    pub const L_PAREN: u32 = 3;    // (
    pub const R_PAREN: u32 = 4;    // )
    pub const L_BRACE: u32 = 1;    // {
    pub const R_BRACE: u32 = 2;    // }
    pub const COMMA: u32 = 9;      // ,

    pub const CLASS: u32 = 47;     // class / Клас
    pub const STRUCT: u32 = 48;    // struct / Структура
    
    // Розділювачі
    pub const COLON: u32 = 7;      // :
    pub const INT_TYPE: u32 = 71;    // int / ціле
    pub const FLOAT_TYPE: u32 = 72;  // float / дійсне
    pub const STRING_TYPE: u32 = 73; // string / рядок
    pub const BOOL_TYPE: u32 = 75;   // bool / булеве
    // src/crates/lexer/src/tokenizer.rs
    
    // --- NATIVE FUNCTIONS (300+) ---
    pub const PRINT: u32 = 300;
    pub const INPUT: u32 = 301;
    pub const LEN: u32 = 302;
    pub const DOT: u32 = 10;
}


#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: String,
    pub std_token_id: u32,
    pub lexem: String,
}

pub struct Parser {
    db: DB,
}

impl Parser {
    pub fn new(db: DB) -> Self {
        info!("Parser initialized.");
        Parser { db }
    }

    pub fn parse(&mut self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                c if c.is_whitespace() => { chars.next(); }
                
                // --- ДОДАЙ ЦЕЙ БЛОК ДЛЯ КОМЕНТАРІВ ---
                '/' => {
                    chars.next(); // Пропускаємо перший '/'
                    if let Some(&next) = chars.peek() {
                        if next == '/' {
                            // Це коментар! Пропускаємо все до кінця рядка
                            while let Some(&comment_char) = chars.peek() {
                                if comment_char == '\n' {
                                    break;
                                }
                                chars.next();
                            }
                            continue; // Йдемо на нову ітерацію while
                        }
                    }
                    // Якщо це не коментар, значить це оператор ділення "/"
                    // Повертаємо токен "/" (ID 16)
                    tokens.push(self.create_token_from_word("/"));
                }
                // --------------------------------------

                '\'' | '"' => {
                    tokens.push(self.read_string(&mut chars, c));
                }
                c if !is_separator(c) => {
                    let word = self.read_word(&mut chars);
                    tokens.push(self.create_token_from_word(&word));
                }
                _ => {
                    let char_str = chars.next().unwrap().to_string();
                    tokens.push(self.create_token_from_word(&char_str));
                }
            }
        }
        tokens
    }

    fn read_string(&self, chars: &mut Peekable<Chars>, quote_type: char) -> Token {
        let mut value = String::new();
        chars.next(); 
        value.push(quote_type);
        let mut content = String::new();

        while let Some(&c) = chars.peek() {
            if c == quote_type {
                chars.next();
                value.push_str(&content);
                value.push(quote_type);
                return Token {
                    value,
                    token_type: "Literal".to_string(),
                    std_token_id: std_ids::STRING_LITERAL, // ID 73
                    lexem: content,
                };
            }
            content.push(chars.next().unwrap());
        }
        
        Token {
            value: content.clone(),
            token_type: "Error".to_string(),
            std_token_id: 0, 
            lexem: content,
        }
    }

    fn read_word(&self, chars: &mut Peekable<Chars>) -> String {
        let mut word = String::new();
        while let Some(&c) = chars.peek() {
            if is_separator(c) || c.is_whitespace() { break; }
            word.push(chars.next().unwrap());
        }
        word
    }

    fn create_token_from_word(&self, lexeme_str: &str) -> Token {
        let db_result = self.db.select_lexem(lexeme_str);

        if let Some((all_tbl, std_tbl)) = db_result.first() {
            Token {
                value: lexeme_str.to_string(),
                token_type: std_tbl.lexem_type.clone(),
                std_token_id: all_tbl.std_lexem as u32, // ID 53, 13, 33 etc.
                lexem: all_tbl.lexem.clone(),
            }
        } else {
            let (guessed_type, guessed_id) = self.guess_type_and_id(lexeme_str);
            Token {
                value: lexeme_str.to_string(),
                token_type: guessed_type,
                std_token_id: guessed_id,
                lexem: lexeme_str.to_string(),
            }
        }
    }

    fn guess_type_and_id(&self, lexeme: &str) -> (String, u32) {
        let first_char = lexeme.chars().next().unwrap_or(' ');
        if first_char.is_numeric() {
            if lexeme.contains('.') {
                ("Literal".to_string(), std_ids::FLOAT_LITERAL) // ID 72
            } else {
                ("Literal".to_string(), std_ids::INT_LITERAL)   // ID 71
            }
        } else {
            ("Identifier".to_string(), std_ids::IDENTIFIER)     // ID 70
        }
    }
}

fn is_separator(c: char) -> bool {
    matches!(c, '.' | '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' | ':' | '+' | '-' | '*' | '=' | '\'' | '"')
}