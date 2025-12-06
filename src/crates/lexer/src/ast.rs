use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    StringLiteral(String),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Call {
        func_name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VariableDeclaration { name: String, value: Expr },
    FunctionDeclaration { name: String, body: Vec<Stmt> },
    Expression(Expr),
}

pub struct AstParser {
    tokens: Vec<Token>,
    current: usize,
}

impl AstParser {
    pub fn new(tokens: Vec<Token>) -> Self {
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
        if self.match_values(&["змінна", "нехай", "var", "let"]) {
            return self.var_declaration();
        }
        if self.match_values(&["функція", "fn", "func"]) {
            return self.function_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume_identifier("Expect variable name.").clone();

        let initializer = if self.match_values(&["="]) {
            self.expression()
        } else {
            Expr::Number(0.0)
        };

        Stmt::VariableDeclaration {
            name,
            value: initializer,
        }
    }

    fn function_declaration(&mut self) -> Stmt {
        let name = self.consume_identifier("Expect function name.").clone();
        self.consume_val("(", "Expect '(' after function name.");
        self.consume_val(")", "Expect ')' after parameters.");
        self.consume_val("{", "Expect '{' before function body.");
        let body = self.block();
        Stmt::FunctionDeclaration { name, body }
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.check_val("}") && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume_val("}", "Expect '}' after block.");
        statements
    }

    fn statement(&mut self) -> Stmt {
        let expr = self.expression();
        Stmt::Expression(expr)
    }

    fn expression(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_values(&["+", "-"]) {
            let operator = self.previous().value.clone();
            let right = self.factor();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.primary();
        while self.match_values(&["*", "/"]) {
            let operator = self.previous().value.clone();
            let right = self.primary();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn primary(&mut self) -> Expr {
        if self.match_values(&["("]) {
            let expr = self.expression();
            self.consume_val(")", "Expect ')' after expression.");
            return expr;
        }

        let token = self.peek().clone();
        self.advance();

        if token.token_type == "Number" {
            let val = token.value.parse::<f64>().unwrap_or(0.0);
            return Expr::Number(val);
        }

        if token.token_type == "String" {
            return Expr::StringLiteral(token.lexem);
        }

        if token.token_type == "Identifier" {
            if self.match_values(&["("]) {
                return self.finish_call(token.value);
            }
            return Expr::Identifier(token.value);
        }

        panic!(
            "Expect expression. Got unexpected token: {:?} (type: {})",
            token.value, token.token_type
        );
    }

    fn finish_call(&mut self, name: String) -> Expr {
        let mut args = Vec::new();
        if !self.check_val(")") {
            loop {
                args.push(self.expression());
                if !self.match_values(&[","]) {
                    break;
                }
            }
        }
        self.consume_val(")", "Expect ')' after arguments.");
        Expr::Call {
            func_name: name,
            args,
        }
    }

    fn match_values(&mut self, values: &[&str]) -> bool {
        for val in values {
            if self.check_val(val) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check_val(&self, val: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().value == val
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

    fn consume_val(&mut self, val: &str, message: &str) -> &Token {
        if self.check_val(val) {
            return self.advance();
        }
        panic!(
            "{} Expected: '{}', Got: '{}' ({})",
            message,
            val,
            self.peek().value,
            self.peek().token_type
        );
    }

    fn consume_identifier(&mut self, message: &str) -> String {
        if self.peek().token_type == "Identifier" {
            return self.advance().value.clone();
        }
        panic!(
            "{} Expected Identifier, Got: {} ({})",
            message,
            self.peek().value,
            self.peek().token_type
        );
    }
}