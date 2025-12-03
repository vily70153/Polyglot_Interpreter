// src/parser.rs
use crate::ast::{Expr, Stmt};
// Импортируй свои типы токенов. 
// Я предполагаю, что они лежат в main или lexer. 
// Замени `crate::lexer::...` на правильный путь к твоим структурам Token и TokenType.
use lexer::tokenizer::Token; 

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        if self.match_value("змінна") || self.match_value("Змінна") {
            return self.var_declaration();
        }
        
        self.statement()
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume_identifier("Очікується назва змінної");

        let value = self.expression();
        Stmt::VarDeclaration { name, value }
    }

    fn statement(&mut self) -> Stmt {
        let expr = self.expression();
        Stmt::Expression(expr)
    }

    fn expression(&mut self) -> Expr {
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        // Число
        if self.match_type(TokenType::Number) {
            let val: f64 = self.previous().value.parse().unwrap_or(0.0);
            return Expr::Number(val);
        }

        // Идентификатор (имя переменной)
        if self.match_type(TokenType::Identifier) {
            return Expr::Variable(self.previous().value.clone());
        }

        // Обработка строк (починка разбитых токенов: ' + текст + ')
        if self.match_value("'") {
            let str_val = self.consume_identifier("Очікується текст рядка").clone();
            if !self.match_value("'") {
                panic!("Рядок повинен закінчуватися кавичкою"); // Лучше использовать Result, но для простоты panic
            }
            return Expr::StringLiteral(str_val);
        }

        panic!("Невідомий токен: {:?}", self.peek());
    }

    // --- Вспомогательные методы ---

    fn match_type(&mut self, t_type: TokenType) -> bool {
        if self.check_type(t_type) {
            self.advance();
            return true;
        }
        false
    }

    fn match_value(&mut self, val: &str) -> bool {
        if self.is_at_end() { return false; }
        if self.peek().value == val {
            self.advance();
            return true;
        }
        false
    }

    fn check_type(&self, t_type: TokenType) -> bool {
        if self.is_at_end() { return false; }
        // Тут нужно сравнить типы. Зависит от того, как у тебя реализован PartialEq для TokenType
        // Если TokenType это enum без данных, то так:
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(&t_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume_identifier(&mut self, error_msg: &str) -> String {
        if self.check_type(TokenType::Identifier) {
            return self.advance().value.clone();
        }
        // Если лексер пометил "тетстовий" как Identifier, оно зайдет сюда.
        // Если вдруг токен строки ' не Identifier, добавь проверку.
        panic!("{}", error_msg);
    }
}