use db::db::DB;
use regex::Regex;
use std::fmt;

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
        let re = Regex::new(r"[a-zA-Zа-яА-ЯіїєґІЇЄҐ0-9_]+|[^\s\w]").unwrap();

        for mat in re.find_iter(input) {
            let lexeme_str = mat.as_str();
            
            let db_result = self.db.select_lexem(lexeme_str);

            let token = if let Some((all_tbl, std_tbl)) = db_result.first() {
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
            };

            tokens.push(token);
        }

        tokens
    }

    fn guess_type(&self, lexeme: &str) -> String {
        if lexeme.chars().all(|c| c.is_numeric()) {
            "Number".to_string()
        } else if lexeme.chars().next().map_or(false, |c| c.is_alphabetic()) {
            "Identifier".to_string()
        } else {
            "UnknownSymbol".to_string()
        }
    }
}