use std::fmt;

use db::db;
use regex::Regex;

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub token_type: String,
}

pub struct Parser {
    db: db::DB,
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Parser {
    pub fn new(db: db::DB) -> Self {
        Parser { db }
    }

    pub fn parse(&mut self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        let re = Regex::new(r"[a-zA-Zа-яА-ЯіїєґІЇЄҐ0-9_]+|[^\s\w]").unwrap();

        for mat in re.find_iter(input) {
            let lexeme_str = mat.as_str();
            
            let db_result = self.db.select_lexem(lexeme_str);
            let token_type: String = if !db_result.is_empty() {
                db_result[0].clone()
            } else {
                self.guess_type(lexeme_str)
            };

            tokens.push(Token {
                value: lexeme_str.to_string(),
                token_type,
            });
        }

        tokens
    }

    fn guess_type(&self, lexeme: &str) -> String {
        if lexeme.chars().all(|c| c.is_numeric()) {
            "Number".to_string()
        } else if lexeme.chars().next().unwrap().is_alphabetic() {
            "Identifier".to_string()
        } else {
            "UnknownSymbol".to_string()
        }
    }
}