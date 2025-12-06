use db::db::DB;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: String,
    pub lang_name: String,
    pub lexem: String,
    pub lexem_type: String,
}

pub struct Parser {
    db: DB,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.value, self.token_type)
    }
}

impl Parser {
    pub fn new(db: DB) -> Self {
        Parser { db }
    }

    pub fn parse(&mut self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                c if c.is_whitespace() => {
                    chars.next();
                }
                '\'' | '"' => {
                    let token = self.read_string(&mut chars, c);
                    tokens.push(token);
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
                    token_type: "StringLiteral".to_string(),
                    lang_name: "Common".to_string(),
                    lexem: content,
                    lexem_type: "String".to_string(),
                };
            }
            content.push(chars.next().unwrap());
        }

        Token {
            value: format!("{}{}", quote_type, content),
            token_type: "UnterminatedString".to_string(),
            lang_name: "Error".to_string(),
            lexem: content,
            lexem_type: "Error".to_string(),
        }
    }

    fn read_word(&self, chars: &mut Peekable<Chars>) -> String {
        let mut word = String::new();
        while let Some(&c) = chars.peek() {
            if is_separator(c) || c.is_whitespace() {
                break;
            }
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
                lang_name: all_tbl.lang_name.clone(),
                lexem: all_tbl.lexem.clone(),
                lexem_type: std_tbl.lexem_type.clone(),
            }
        } else {
            let guessed_type = self.guess_type(lexeme_str);
            Token {
                value: lexeme_str.to_string(),
                token_type: guessed_type,
                lang_name: "Unknown".to_string(),
                lexem: lexeme_str.to_string(),
                lexem_type: "N/A".to_string(),
            }
        }
    }

    fn guess_type(&self, lexeme: &str) -> String {
        let first_char = lexeme.chars().next().unwrap_or(' ');
        if first_char.is_numeric() {
            "Number".to_string()
        } else if first_char.is_alphabetic() || first_char == '_' {
            "Identifier".to_string()
        } else {
            "Symbol".to_string()
        }
    }
}

fn is_separator(c: char) -> bool {
    matches!(c, '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' | ':' | '+' | '-' | '*' | '/' | '=' | '\'' | '"')
}