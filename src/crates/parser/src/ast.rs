#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    StringLiteral(String),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDeclaration { name: String, value: Expr },
    FunctionDecl { name: String }, 
    Expression(Expr),
}