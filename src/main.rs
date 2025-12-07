use db::db::DB;
use lexer::{tokenizer, ast::AstParser};
use shared::logging;
use dotenvy::dotenv;
use tracing::info;
use std::env;
use tokio;

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
        структура Test { 
                user_id: ціле
        }
        функція Func ( Test Дані ) {змінна myVariable = Дані}
    ";
    
    let statements = AstParser::new(lexer.parse(input_code)).parse();

    for stmt in statements {
        println!("{:#?}", stmt);
    }
}