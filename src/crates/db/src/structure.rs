use types::type_system::TokenKind;

#[derive(Clone)]
pub struct StdLexemTBL {
  pub id: usize,
  pub name: &'static str,
  pub lexem_type: TokenKind,
}

#[derive(Clone)]
pub struct AccessedLexemesTBL {
  pub id: usize,
  pub lang_name: &'static str,
  pub lexeme: &'static str,
  pub type_info: &'static str,
  pub std_lexem: StdLexemTBL
}
