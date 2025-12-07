#[cfg(test)]
mod tests {
    use lexer::ast::{AstParser, Stmt, Expr, DataType};
    use lexer::tokenizer::std_ids;
    use crate::test_helper::*;

    #[test]
    fn test_var_declaration() {
        let tokens = vec![
            t("let", ID_VAR),
            t("x", ID_NAME),
            t("=", ID_EQ),
            t("10", ID_NUM),
        ];

        let mut parser = AstParser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.len(), 1);
        
        if let Stmt::VariableDeclaration { name, value } = &result[0] {
            assert_eq!(name, "x");
            if let Expr::Number(val) = value {
                assert_eq!(*val, 10.0);
            } else {
                panic!("Expected Number value");
            }
        } else {
            panic!("Expected VariableDeclaration");
        }
    }

    #[test]
    fn test_struct_declaration() {
        let tokens = vec![
            t("struct", ID_STRUCT),
            t("User", ID_NAME),
            t("{", ID_L_BRACE),
            t("id", ID_NAME),
            t(":", ID_COLON),
            t("int", ID_INT),
            t("}", ID_R_BRACE),
        ];

        let mut parser = AstParser::new(tokens);
        let result = parser.parse();

        assert_eq!(result.len(), 1);

        if let Stmt::StructDeclaration { name, fields } = &result[0] {
            assert_eq!(name, "User");
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].0, "id");
            assert_eq!(fields[0].1, DataType::Int);
        } else {
            panic!("Expected StructDeclaration");
        }
    }

    #[test]
    fn test_function_declaration() {
        let tokens = vec![
            t("fn", ID_FUNC),
            t("main", ID_NAME),
            t("(", ID_L_PAREN),
            t("float", std_ids::FLOAT_TYPE),
            t("arg", ID_NAME),
            t(")", ID_R_PAREN),
            t("{", ID_L_BRACE),
            t("}", ID_R_BRACE),
        ];

        let mut parser = AstParser::new(tokens);
        let result = parser.parse();

        if let Stmt::FunctionDeclaration { name, params, body } = &result[0] {
            assert_eq!(name, "main");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].0, "arg");
            assert_eq!(params[0].1, DataType::Float);
            assert!(body.is_empty());
        } else {
            panic!("Expected FunctionDeclaration");
        }
    }

    #[test]
    fn test_complex_integration() {
        let tokens = vec![
            t("структура", ID_STRUCT),
            t("Test", ID_NAME),
            t("{", ID_L_BRACE),
            t("user_id", ID_NAME),
            t(":", ID_COLON),
            t("ціле", ID_INT),
            t("}", ID_R_BRACE),

            t("функція", ID_FUNC),
            t("Func", ID_NAME),
            t("(", ID_L_PAREN),
            t("Test", ID_NAME),
            t("Дані", ID_NAME),
            t(")", ID_R_PAREN),
            t("{", ID_L_BRACE),
            
            t("змінна", ID_VAR),
            t("myVariable", ID_NAME),
            t("=", ID_EQ),
            t("Дані", ID_NAME),
            t("}", ID_R_BRACE),
        ];

        let mut parser = AstParser::new(tokens);
        let statements = parser.parse();

        assert_eq!(statements.len(), 2);

        match &statements[0] {
            Stmt::StructDeclaration { name, fields } => {
                assert_eq!(name, "Test");
                assert_eq!(fields[0].0, "user_id");
                assert_eq!(fields[0].1, DataType::Int);
            },
            _ => panic!("Перший Statement має бути структурою"),
        }

        match &statements[1] {
            Stmt::FunctionDeclaration { name, params, body } => {
                assert_eq!(name, "Func");
                
                assert_eq!(params.len(), 1);
                assert_eq!(params[0].0, "Дані");
                match &params[0].1 {
                    DataType::Custom(type_name) => assert_eq!(type_name, "Test"),
                    _ => panic!("Параметр має бути Custom типу"),
                }

                assert_eq!(body.len(), 1);
                match &body[0] {
                    Stmt::VariableDeclaration { name, value } => {
                        assert_eq!(name, "myVariable");
                        match value {
                            Expr::Identifier(val_name) => assert_eq!(val_name, "Дані"),
                            _ => panic!("Значення змінної має бути ідентифікатором"),
                        }
                    },
                    _ => panic!("Тіло функції має містити оголошення змінної"),
                }
            },
            _ => panic!("Другий Statement має бути функцією"),
        }
    }
}