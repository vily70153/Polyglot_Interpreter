use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[derive(Queryable)] 
pub struct LexemeTypeResult {
    pub lexem_type: String,
}


type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct DB {
    pool: Pool,
}

impl DB {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        DB { pool }
    }

    pub fn select_lexem(&self, search_lexem: &str) -> Vec<String> {
        let mut conn = self.pool.get().expect("Failed to get connection");

        use crate::schema::AllLexemsTBL::dsl as al;
        use crate::schema::StdLexemeTBL::dsl as std;

        let results = al::AllLexemsTBL
            .inner_join(std::StdLexemeTBL.on(al::std_lexem.eq(std::id)))
            .filter(al::lexem.eq(search_lexem))
            .select(std::lexem_type)
            .load::<String>(&mut conn);

        match results {
            Ok(rows) => rows,
            Err(_) => vec![],
        }
    }
}
