use dotenvy::dotenv;
use tracing::info;
use std::env;
use tokio;
use clap::Parser;
use interpreter::interpreter::Interpreter;
use shared::{logging, configuration::CONFIG};
use lexer::{tokenizer, ast::AstParser};
use db::db::DB;

#[derive(Parser, Debug)]
#[command(name = "usqlrepl")]
#[command(version, about = "Your REPL")]
struct Args {
    #[arg(long)]
    no_logging: bool,
    #[arg(long)]
    lang: Option<String>,
    #[arg()]
    extra: Vec<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C, shutting down...");
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    let args = Args::parse();

    if !args.no_logging {
        logging::init_logging();
    }

    info!("Application started.");

    let config = CONFIG.clone();
    let mut _current_lang = config.lang;
    if let Some(cli_lang) = args.lang {
        _current_lang = cli_lang;
    }
    info!("Selected language: {}", _current_lang);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _db = DB::new(&db_url);
    
    let mut lexer = tokenizer::Parser::new(_db);

    let input_code = "
        структура Test { user_id: ціле }
        
        функція Func ( Test Дані ) {
            змінна myVariable = Дані.user_id
            друк(myVariable)
        }
        
        Func(Test(1))
    ";

    let statements = AstParser::new(lexer.parse(input_code)).parse();
    let mut interp = Interpreter::new();
    interp.interpret(statements);
}