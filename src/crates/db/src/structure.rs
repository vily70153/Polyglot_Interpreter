use types::type_system::TokenKind;


/*
  Table of Standart tokens;
  Standart tokens - all tokens thau User can use
  for creating self language tokens;

  For example:
    id: 1
    name: "{"
    lexem_type: Delimiter
*/

#[derive(Clone, PartialEq)]
pub struct StdLexemTBL {
  pub id: i32,
  pub name: &'static str,
  pub lexem_type: TokenKind,
}


/*
  Table of all accessed tokens, created by User;

  Created by StdLexemTBL;
*/

#[derive(Clone, PartialEq)]
pub struct AccessedLexemesTBL {
  pub id: i32,
  pub lang_name: &'static str,
  pub lexeme: &'static str,
  pub type_info: Option<&'static str>,
  pub std_lexem: StdLexemTBL
}