use db::{self, db::DB};
use lexer::tokenizer;

use dotenvy::dotenv;
use std::env;

mod logging;

fn main() {
    dotenv().ok();
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C, shutting down...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    logging::init_logging();

    let mut _db = DB::new(
        env::var("DATABASE_URL")
                     .expect("DATABASE_URL must be set").as_str(), );
    let mut parser = tokenizer::Parser::new(_db);
    let tokens = parser.parse("1 2234 131 'тетстовий текст' функція нехай змінна змінна число");
    for el in tokens {
        println!("{:?}", el);
    }
}
