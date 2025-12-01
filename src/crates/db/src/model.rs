use diesel::prelude::*;
use crate::schema::{AllLexemsTBL, StdLexemeTBL};


#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = AllLexemsTBL)]
pub struct AllLexem {
    pub id: u32,
    pub lang_name: String,
    pub lexem: String,
    pub type_info: Option<String>,
    pub std_lexem: u32,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = StdLexemeTBL)]
pub struct StdLexeme {
    pub id: u32,
    pub name: String,
    pub lexem_type: String,
}