use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use lexer::ast::{Stmt, Expr, DataType};
use tracing::{info, error, debug};
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Bool(bool),
    StructInstance {
        type_name: String,
        fields: HashMap<String, RuntimeValue>,
    },
    Function {
        name: String,
        params: Vec<(String, DataType)>,
        body: Vec<Stmt>,
    },
    Null,
    Void,
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, RuntimeValue>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(parent: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            values: HashMap::new(),
            parent,
        }
    }

    // Объявить новую переменную
    pub fn define(&mut self, name: String, value: RuntimeValue) {
        self.values.insert(name, value);
    }

    // Получить значение переменной (ищет в текущей области, затем в родительской)
    pub fn get(&self, name: &str) -> Option<RuntimeValue> {
        if let Some(val) = self.values.get(name) {
            return Some(val.clone());
        }
        // Если нет у нас, ищем у родителя
        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }
        None
    }
}

// --- Interpreter ---
pub struct Interpreter {
    // Global scope или текущий scope
    pub env: Rc<RefCell<Environment>>,
    // Хранилище определений структур (просто шаблоны)
    pub struct_definitions: HashMap<String, Vec<(String, DataType)>>,
}

impl Interpreter {
    pub fn new() -> Self {
        // Создаем глобальное окружение
        let global_env = Rc::new(RefCell::new(Environment::new(None)));
        Interpreter {
            env: global_env,
            struct_definitions: HashMap::new(),
        }
    }

    // Главная точка входа
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        info!("--- Interpreter Started ---");
        for stmt in statements {
            self.execute(stmt);
        }
        info!("--- Interpreter Finished ---");
    }

    // Выполнение инструкций (Statements)
    fn execute(&mut self, stmt: Stmt) -> RuntimeValue {
        match stmt {
            Stmt::VariableDeclaration { name, value } => {
                let val = self.evaluate(value);
                debug!("Var Decl: {} = {:?}", name, val);
                self.env.borrow_mut().define(name, val);
                RuntimeValue::Void
            },
            Stmt::FunctionDeclaration { name, params, body } => {
                debug!("Func Decl: {}", name);
                let func_obj = RuntimeValue::Function {
                    name: name.clone(),
                    params,
                    body,
                };
                self.env.borrow_mut().define(name, func_obj);
                RuntimeValue::Void
            },
            Stmt::StructDeclaration { name, fields } => {
                debug!("Struct Decl: {}", name);
                // Сохраняем определение структуры отдельно
                self.struct_definitions.insert(name, fields);
                RuntimeValue::Void
            },
            Stmt::Expression(expr) => {
                self.evaluate(expr)
            }
        }
    }

    // Вычисление выражений (Expressions)
    fn evaluate(&mut self, expr: Expr) -> RuntimeValue {
        match expr {
            Expr::Number(n) => RuntimeValue::Number(n),
            Expr::StringLiteral(s) => RuntimeValue::String(s),
            Expr::Identifier(name) => {
                let env = self.env.borrow();
                match env.get(&name) {
                    Some(val) => val,
                    None => {
                        error!("Runtime Error: Undefined variable '{}'", name);
                        RuntimeValue::Null
                    }
                }
            },
            Expr::BinaryOp { left, op, right } => {
                let l = self.evaluate(*left);
                let r = self.evaluate(*right);
                self.apply_binary_op(l, op, r)
            },
            Expr::Call { func_name, args } => {
                self.call_function(func_name, args)
            },
        }
    }
        
        fn call_function(&mut self, func_name: String, args: Vec<Expr>) -> RuntimeValue {
            
            // --- NATIVE FUNCTIONS HANDLER ---
            // Проверяем имя функции (поддерживаем и EN, и UA версии, 
            // так как парсер передает имя как строку)
            match func_name.as_str() {
                "print" | "друк" => {
                    let mut output = Vec::new();
                    for arg in args {
                        let val = self.evaluate(arg);
                        match val {
                            RuntimeValue::Number(n) => output.push(n.to_string()),
                            RuntimeValue::String(s) => output.push(s),
                            RuntimeValue::Bool(b) => output.push(b.to_string()),
                            RuntimeValue::Null => output.push("null".to_string()),
                            RuntimeValue::Void => output.push("void".to_string()),
                            RuntimeValue::StructInstance { type_name, .. } => {
                                output.push(format!("[Instance of {}]", type_name))
                            }
                            RuntimeValue::Function { name, .. } => {
                                output.push(format!("[Function {}]", name))
                            }
                        }
                    }
                    println!("{}", output.join(" "));
                    return RuntimeValue::Void;
                },
                
                "input" | "ввід" => {
                    // Если передали аргумент, выводим его как подсказку (prompt)
                    if let Some(arg) = args.first() {
                         let prompt = self.evaluate(arg.clone());
                         if let RuntimeValue::String(s) = prompt {
                             print!("{}", s);
                             // Сбрасываем буфер, чтобы текст появился до ввода
                             io::stdout().flush().unwrap(); 
                         }
                    }
    
                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).expect("Failed to read input");
                    // Обрезаем символ новой строки (\n) в конце
                    return RuntimeValue::String(buffer.trim().to_string());
                },
    
                "len" | "довжина" => {
                    if args.len() != 1 {
                        error!("Function 'len' expects 1 argument");
                        return RuntimeValue::Null;
                    }
                    let val = self.evaluate(args[0].clone());
                    if let RuntimeValue::String(s) = val {
                        return RuntimeValue::Number(s.len() as f64);
                    } else {
                        error!("Function 'len' expects a String");
                        return RuntimeValue::Number(0.0);
                    }
                },
                
                // Если это не нативная функция, идем дальше к пользовательским
                _ => {} 
            }
    
            // --- USER DEFINED FUNCTIONS ---
            let func_val = {
                let env = self.env.borrow();
                env.get(&func_name)
            };
            
            // ... (ваш старый код вызова пользовательских функций) ...
            match func_val {
                Some(RuntimeValue::Function { params, body, .. }) => {
                    // Ваша логика создания Scope и выполнения body
                    // ... (код который я давал в прошлом ответе)
                    
                    // КОПИЯ ЛОГИКИ ДЛЯ КОНТЕКСТА:
                    if args.len() != params.len() {
                        error!("Arg count mismatch for '{}'", func_name);
                        return RuntimeValue::Null;
                    }
                    let mut evaluated_args = Vec::new();
                    for arg in args { evaluated_args.push(self.evaluate(arg)); }
                    
                    let func_env = Rc::new(RefCell::new(crate::interpreter::Environment::new(Some(self.env.clone()))));
                    for (i, (param_name, _)) in params.iter().enumerate() {
                        func_env.borrow_mut().define(param_name.clone(), evaluated_args[i].clone());
                    }
                    
                    let previous_env = self.env.clone();
                    self.env = func_env;
                    
                    let mut result = RuntimeValue::Void;
                    for stmt in body {
                        result = self.execute(stmt);
                    }
                    
                    self.env = previous_env;
                    result
                },
                _ => {
                    error!("Undefined function '{}'", func_name);
                    RuntimeValue::Null
                }
            }
        }

    fn apply_binary_op(&self, left: RuntimeValue, op: String, right: RuntimeValue) -> RuntimeValue {
        match (left, right) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => match op.as_str() {
                "+" => RuntimeValue::Number(a + b),
                "-" => RuntimeValue::Number(a - b),
                "*" => RuntimeValue::Number(a * b),
                "/" => RuntimeValue::Number(a / b),
                _ => RuntimeValue::Null,
            },
            // Можно добавить конкатенацию строк
            (RuntimeValue::String(a), RuntimeValue::String(b)) => {
                if op == "+" { RuntimeValue::String(format!("{}{}", a, b)) } else { RuntimeValue::Null }
            },
            _ => {
                error!("Invalid operands for operator {}", op);
                RuntimeValue::Null
            }
        }
    }
}