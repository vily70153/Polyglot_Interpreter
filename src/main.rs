use db::{self, db::DB};
use lexer::tokenizer;

use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let mut _db = DB::new(
        env::var("DATABASE_URL")
                     .expect("DATABASE_URL must be set").as_str(), );
    let mut parser = tokenizer::Parser::new(_db);
    let tokens = parser.parse("1 2234 131 функція нехай змінна змінна число");
    for el in tokens {
        println!("{:?}", el);
    }
}
