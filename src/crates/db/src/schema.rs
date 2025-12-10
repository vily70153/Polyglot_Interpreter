#![allow(non_snake_case)]
// @generated automatically by Diesel CLI.
diesel::table! {
    AllLexemsTBL (id) {
        id -> Unsigned<Integer>,
        #[max_length = 64]
        lang_name -> Varchar,
        #[max_length = 128]
        lexem -> Varchar,
        #[max_length = 128]
        type_info -> Nullable<Varchar>,
        std_lexem -> Unsigned<Integer>,
    }
}

diesel::table! {
    StdLexemeTBL (id) {
        id -> Unsigned<Integer>,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        lexem_type -> Varchar,
    }
}

diesel::joinable!(AllLexemsTBL -> StdLexemeTBL (std_lexem));

diesel::allow_tables_to_appear_in_same_query!(
    AllLexemsTBL,
    StdLexemeTBL,
);
