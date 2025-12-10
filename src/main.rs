use dotenvy::dotenv;
use tracing::{info, error};
use std::env;
use tokio;
use clap::Parser;
use interpreter::interpreter::Interpreter;
use shared::{logging, configuration::CONFIG};
use lexer::{tokenizer, ast::AstParser};
use db::db::DB;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "usqlrepl")]
#[command(version, about = "Your REPL")]
struct Args {
    #[arg(long)]
    no_logging: bool,
    #[arg(long)]
    lang: Option<String>,
    #[arg(long)]
    path: Option<String>,
    
    #[arg(help = "Path to the source file")]
    file_path: Option<String>, 

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
    let mut _config_path = config.path_config;

    if let Some(cli_lang) = args.lang {
        _current_lang = cli_lang;
    }
    if let Some(conf_path) = args.path {
        _config_path = conf_path;
    }

    info!("Selected language: {}", _current_lang);
    info!("Config path: {}", _config_path);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _db = DB::new(&db_url);
    
    let mut lexer = tokenizer::Parser::new(_db);

    let input_code = if let Some(path) = args.file_path {
        info!("Reading source code from file: {}", path);
        fs::read_to_string(&path).expect("Failed to read source file")
    } else {
        error!("No file provided, running default test code.");
        return;
    };

    let statements = AstParser::new(lexer.parse(&input_code)).parse();
    let mut interp = Interpreter::new();
    interp.interpret(statements);
}