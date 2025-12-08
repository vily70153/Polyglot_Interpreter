use db::db::DB;
use lexer::{tokenizer, ast::{AstParser, Stmt, Expr}};
use shared::logging;
use dotenvy::dotenv;
use tracing::info;
use std::env;
use tokio;
use interpreter::interpreter::Interpreter;



#[tokio::main]
async fn main() {
    dotenv().ok();
    
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C, shutting down...");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    logging::init_logging();
    info!("Application started.");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _db = DB::new(&db_url);

    let mut lexer = tokenizer::Parser::new(_db);
    let input_code = "
        структура Test { user_id: ціле }
        функція Func ( Test Дані ) {
             змінна myVariable = Дані
             друк(myVariable)  // <-- Вызов нашей новой нативной функции
        }
    ";
    
    let mut statements = AstParser::new(lexer.parse(input_code)).parse();

    let mut interp = Interpreter::new();
    
    statements.push(Stmt::Expression(Expr::Call {
        func_name: "Func".to_string(),
        args: vec![Expr::Number(999.0)], 
    }));

    interp.interpret(statements);
}