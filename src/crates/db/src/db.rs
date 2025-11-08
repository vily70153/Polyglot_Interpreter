use postgres::{Client, NoTls};

pub struct DB<'db> {
    pub client: &'db mut Client,
}

impl <'db> DB<'db> {
    pub fn new(host: &str, user: &str) -> Self {
        let conn_params = format!("host={}, user={}", host, user);
        let mut client = Client::connect(&conn_params, NoTls)
            .expect("Cannot connect to database");

        Self { client: &mut client }
    }

    pub fn select_lexem(&mut self, lexem: &str) -> Vec<String> {
        let rows = self.client
            .query("SELECT value FROM accessedlexems WHERE lexeme = $1", &[&lexem])
            .expect("Query failed");

        rows.into_iter()
            .map(|r| r.get::<_, String>(0))
            .collect()
    }
}
