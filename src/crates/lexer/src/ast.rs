

#[derive(Debug)]
enum Expr {
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

#[derive(Debug)]
enum Stmt {
    VariableDeclaration {
        name: String,
        value: Expr,
    },
    FunctionDeclaration {
        name: String,
        body: Vec<Stmt>,
    },
    Expression(Expr),
}