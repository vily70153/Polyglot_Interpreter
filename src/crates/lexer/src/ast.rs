use crate::tokenizer::{Token, std_ids};
use tracing::{info, trace, error};


#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    StringLiteral(String),
    Identifier(String),
    BinaryOp { left: Box<Expr>, op: String, right: Box<Expr> },
    Call {
        func_id: u32,
        func_name: String,
        args: Vec<Expr>,
    },
    MemberAccess {
        object: Box<Expr>,
        member: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VariableDeclaration { name: String, value: Expr },
    FunctionDeclaration { 
        name: String, 
        params: Vec<(String, DataType)>, 
        body: Vec<Stmt> 
    },
    Expression(Expr),
    StructDeclaration {
        name: String,
        fields: Vec<(String, DataType)>,
    },
}

pub struct AstParser {
    tokens: Vec<Token>,
    current: usize,
}

impl AstParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        info!("AstParser initialized.");
        AstParser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        if self.match_id(std_ids::LET) { return self.var_declaration(); }
        if self.match_id(std_ids::FUNCTION) { return self.function_declaration(); }
        if self.match_id(std_ids::STRUCT) || self.match_id(std_ids::CLASS) {
            return self.struct_declaration();
        }
        self.statement()
    }

    fn parse_type(&mut self) -> DataType {
        let token = self.peek();
        
        let data_type = match token.std_token_id {
            std_ids::INT_TYPE => DataType::Int,
            std_ids::FLOAT_TYPE => DataType::Float,
            std_ids::STRING_TYPE => DataType::String,
            std_ids::BOOL_TYPE => DataType::Bool,
            std_ids::IDENTIFIER => DataType::Custom(token.value.clone()),
            _ => {
                let err = format!("Expected Type, got: {} (ID: {})", token.value, token.std_token_id);
                error!("{}", err);
                panic!("{}", err);
            }
        };

        self.advance();
        data_type
    }

    fn struct_declaration(&mut self) -> Stmt {
        let name = self.consume_id(std_ids::IDENTIFIER, "Expect struct name").value.clone();
        self.consume_id(std_ids::L_BRACE, "Expect '{'");

        let mut fields: Vec<(String, DataType)> = Vec::new();

        while !self.check_id(std_ids::R_BRACE) && !self.is_at_end() {
            let field_name = self.consume_id(std_ids::IDENTIFIER, "Expect field name").value.clone();
            self.consume_id(std_ids::COLON, "Expect ':'");
            
            let field_type = self.parse_type();
            
            trace!("Field: {} -> {:?}", field_name, field_type);
            fields.push((field_name, field_type));
            
            self.match_id(std_ids::COMMA);
        }

        self.consume_id(std_ids::R_BRACE, "Expect '}'");
        Stmt::StructDeclaration { name, fields }
    }

    fn function_declaration(&mut self) -> Stmt {
        let name = self.consume_id(std_ids::IDENTIFIER, "Expect function name").value.clone();
        self.consume_id(std_ids::L_PAREN, "Expect '('");
        
        let mut params: Vec<(String, DataType)> = Vec::new();

        if !self.check_id(std_ids::R_PAREN) {
            loop {
                let param_type = self.parse_type();
                let param_name = self.consume_id(std_ids::IDENTIFIER, "Expect param name").value.clone();

                trace!("Param: {} -> {:?}", param_name, param_type);
                params.push((param_name, param_type));

                if !self.match_id(std_ids::COMMA) { break; }
            }
        }

        self.consume_id(std_ids::R_PAREN, "Expect ')'");
        self.consume_id(std_ids::L_BRACE, "Expect '{'");
        let body = self.block();
        
        Stmt::FunctionDeclaration { name, params, body }
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume_id(std_ids::IDENTIFIER, "Expect variable name").value.clone();
        let initializer = if self.match_id(std_ids::ASSIGN) { self.expression() } else { Expr::Number(0.0) };
        Stmt::VariableDeclaration { name, value: initializer }
    }
    
    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.check_id(std_ids::R_BRACE) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume_id(std_ids::R_BRACE, "Expect '}'");
        statements
    }
    fn statement(&mut self) -> Stmt { Stmt::Expression(self.expression()) }
    fn expression(&mut self) -> Expr { self.term() }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_ids(&[std_ids::PLUS, std_ids::MINUS]) {
            let op = self.previous().value.clone();
            let right = self.factor();
            expr = Expr::BinaryOp { left: Box::new(expr), op, right: Box::new(right) };
        }
        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.primary();
        while self.match_ids(&[std_ids::MULTIPLY, std_ids::DIVIDE]) {
            let op = self.previous().value.clone();
            let right = self.primary();
            expr = Expr::BinaryOp { left: Box::new(expr), op, right: Box::new(right) };
        }
        expr
    }
    fn primary(&mut self) -> Expr {
        if self.match_id(std_ids::L_PAREN) {
            let expr = self.expression();
            self.consume_id(std_ids::R_PAREN, "Expect ')'");
            return expr;
        }

        let token = self.peek().clone();

        if token.std_token_id == std_ids::INT_LITERAL || token.std_token_id == std_ids::FLOAT_LITERAL {
            self.advance();
            return Expr::Number(token.value.parse().unwrap_or(0.0));
        }
        if token.std_token_id == std_ids::STRING_LITERAL {
            self.advance();
            return Expr::StringLiteral(token.lexem);
        }

        let is_identifier_like = 
            token.std_token_id == std_ids::IDENTIFIER || 
            token.std_token_id == std_ids::PRINT ||      
            token.std_token_id == std_ids::INPUT ||      
            token.std_token_id == std_ids::LEN;

        if is_identifier_like {
            self.advance();

            if self.match_id(std_ids::L_PAREN) {
                return self.finish_call(token.std_token_id, token.value);
            }

            let mut expr = Expr::Identifier(token.value);

            while self.match_id(std_ids::DOT) { // ID 10
                let member_name = self.consume_id(std_ids::IDENTIFIER, "Expect field name").value.clone();
                
                expr = Expr::MemberAccess { 
                    object: Box::new(expr),
                    member: member_name 
                };
            }

            return expr;
        }

        let err = format!("Unexpected token: {} (ID: {})", token.value, token.std_token_id);
        error!("{}", err);
        panic!("{}", err);
    }

    fn finish_call(&mut self, func_id: u32, name: String) -> Expr {
        let mut args = Vec::new();
        if !self.check_id(std_ids::R_PAREN) {
            loop {
                args.push(self.expression());
                if !self.match_id(std_ids::COMMA) {
                    break;
                }
            }
        }
        self.consume_id(std_ids::R_PAREN, "Expect ')' after args");
        Expr::Call { func_id, func_name: name, args }
    }
    
    fn match_id(&mut self, id: u32) -> bool { if self.check_id(id) { self.advance(); true } else { false } }
    fn match_ids(&mut self, ids: &[u32]) -> bool { for &id in ids { if self.check_id(id) { self.advance(); return true; } } false }
    fn check_id(&self, id: u32) -> bool { !self.is_at_end() && self.peek().std_token_id == id }
    fn advance(&mut self) -> &Token { if !self.is_at_end() { self.current += 1; } self.previous() }
    fn is_at_end(&self) -> bool { self.current >= self.tokens.len() }
    fn peek(&self) -> &Token { &self.tokens[self.current] }
    fn previous(&self) -> &Token { &self.tokens[self.current - 1] }
    fn consume_id(&mut self, id: u32, msg: &str) -> &Token {
        if self.check_id(id) { return self.advance(); }
        panic!("{} Expected ID: {}, Got: {}", msg, id, self.peek().value);
    }
}