use db::{self, db::DB};
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let mut _db = DB::new(
        env::var("DATABASE_URL")
                     .expect("DATABASE_URL must be set").as_str(), );
    println!("{:?}", _db.select_lexem("змінна"));
}
