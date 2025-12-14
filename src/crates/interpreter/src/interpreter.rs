use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use lexer::ast::{Stmt, Expr, DataType};
use tracing::{info, error, debug};
use std::io::{self, Write};
use lexer::tokenizer::std_ids;

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

    Return(Box<RuntimeValue>),
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

    pub fn define(&mut self, name: String, value: RuntimeValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<RuntimeValue> {
        if let Some(val) = self.values.get(name) {
            return Some(val.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }
        None
    }
}

pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
    pub struct_definitions: HashMap<String, Vec<(String, DataType)>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let global_env = Rc::new(RefCell::new(Environment::new(None)));
        Interpreter {
            env: global_env,
            struct_definitions: HashMap::new(),
        }
    }
    fn is_truthy(val: &RuntimeValue) -> bool {
        match val {
            RuntimeValue::Bool(b) => *b,
            RuntimeValue::Null => false,
            RuntimeValue::Void => false,
            RuntimeValue::Number(n) => *n != 0.0,
            _ => true,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        info!("--- Interpreter Started ---");
        for stmt in statements {
            self.execute(stmt);
        }
        info!("--- Interpreter Finished ---");
    }

    // fn is_truthy(&self, val: &RuntimeValue) -> bool {
    //     match val {
    //         RuntimeValue::Bool(b) => *b,
    //         RuntimeValue::Null => false,
    //         RuntimeValue::Void => false,
    //         RuntimeValue::Number(n) => *n != 0.0, // 0 - false, остальное true
    //         _ => true,
    //     }
    // }

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
                // Зберігаємо визначення структури окремо
                self.struct_definitions.insert(name, fields);
                RuntimeValue::Void
            },
            Stmt::Expression(expr) => {
                self.evaluate(expr)
            },
            Stmt::If { condition, then_branch, else_branch } => {
                let cond_val = self.evaluate(condition);
                // ВИПРАВЛЕНО: Self::is_truthy (без self.)
                if Self::is_truthy(&cond_val) {
                    return self.execute_block(then_branch);
                } else if let Some(else_stmts) = else_branch {
                    return self.execute_block(else_stmts);
                }
                RuntimeValue::Void
            },

            Stmt::While { condition, body } => {
                // ВИПРАВЛЕНО: Self::is_truthy замість self.is_truthy
                // Тепер конфлікту немає, бо ми не позичаємо self для перевірки істини
                while Self::is_truthy(&self.evaluate(condition.clone())) {
                    let result = self.execute_block(body.clone());
                    if let RuntimeValue::Return(_) = result {
                        return result;
                    }
                }
                RuntimeValue::Void
            },
            Stmt::Return { value } => {
                let ret_val = if let Some(expr) = value {
                    self.evaluate(expr)
                } else {
                    RuntimeValue::Null
                };
                return RuntimeValue::Return(Box::new(ret_val));
            },

            Stmt::Expression(expr) => self.evaluate(expr),
        }
    }

    fn execute_block(&mut self, statements: Vec<Stmt>) -> RuntimeValue {
        let block_env = Rc::new(RefCell::new(Environment::new(Some(self.env.clone()))));
        let previous_env = self.env.clone();
        self.env = block_env;

        let mut result = RuntimeValue::Void;
        for stmt in statements {
            result = self.execute(stmt);
            
            if let RuntimeValue::Return(_) = result {
                break;
            }
        }

        self.env = previous_env; // Возвращаем старый scope
        result
    }

    pub fn evaluate(&mut self, expr: Expr) -> RuntimeValue {
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
            Expr::Call { func_id, func_name, args } => {
                self.call_function(func_id, func_name, args)
            },
            Expr::MemberAccess { object, member } => {
                let obj_val = self.evaluate(*object);
                if let RuntimeValue::StructInstance { fields, .. } = obj_val {
                    if let Some(val) = fields.get(&member) {
                        return val.clone();
                    } else {
                        error!("Field '{}' not found in struct instance", member);
                        return RuntimeValue::Null;
                    }
                } else {
                    error!("Cannot access member '{}' of non-struct", member);
                    return RuntimeValue::Null;
                }
            },
        }
    }

    fn call_function(&mut self, func_id: u32, func_name: String, args: Vec<Expr>) -> RuntimeValue {
        match func_id {
            std_ids::PRINT => { // 300
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
                        RuntimeValue::Return(inner_val) => {
                            output.push(format!("{:?}", inner_val))
                        }
                    }
                }
                println!("{}", output.join(" "));
                return RuntimeValue::Void;
            },
            std_ids::INPUT => { // 301
                if let Some(arg) = args.first() {
                    let prompt = self.evaluate(arg.clone());
                    if let RuntimeValue::String(s) = prompt {
                        print!("{}", s);
                        io::stdout().flush().unwrap();
                    }
                }
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).expect("Failed to read input");
                return RuntimeValue::String(buffer.trim().to_string());
            },
            std_ids::LEN => { // 302
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
            _ => {} // Якщо ID не нативний, йдемо далі
        }

        // 2. CONSTRUCTORS
        // Конструктори структур
        if let Some(fields_def) = self.struct_definitions.get(&func_name).cloned() {
            if args.len() != fields_def.len() {
                error!("Constructor '{}' expects {} arguments, got {}", func_name, fields_def.len(), args.len());
                return RuntimeValue::Null;
            }
            
            let mut instance_fields = HashMap::new();
            for (i, (field_name, _)) in fields_def.iter().enumerate() {
                let val = self.evaluate(args[i].clone());
                instance_fields.insert(field_name.clone(), val);
            }
            return RuntimeValue::StructInstance {
                type_name: func_name,
                fields: instance_fields,
            };
        }

        let func_val = {
            let env = self.env.borrow();
            env.get(&func_name)
        };

        match func_val {
            Some(RuntimeValue::Function { params, body, .. }) => {
                if args.len() != params.len() {
                    error!("Arg count mismatch for '{}'. Expected {}, got {}", func_name, params.len(), args.len());
                    return RuntimeValue::Null;
                }

                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.evaluate(arg));
                }

                let func_env = Rc::new(RefCell::new(Environment::new(Some(self.env.clone()))));

                for (i, (param_name, _)) in params.iter().enumerate() {
                    func_env.borrow_mut().define(param_name.clone(), evaluated_args[i].clone());
                }

                let previous_env = self.env.clone(); // Зберігаємо старий
                self.env = func_env;                 // Включаємо новий

                let mut return_value = RuntimeValue::Void; // Значення за замовчуванням

                for stmt in body {
                    let result = self.execute(stmt.clone());
                    
                    if let RuntimeValue::Return(val) = result {
                        return_value = *val;
                        break;
                    }
                }

                self.env = previous_env;
                return return_value;
            },
            _ => {
                error!("Undefined function '{}' (ID: {})", func_name, func_id);
                return RuntimeValue::Null;
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
                
                "<" => RuntimeValue::Bool(a < b),
                ">" => RuntimeValue::Bool(a > b),
                "<=" => RuntimeValue::Bool(a <= b),
                ">=" => RuntimeValue::Bool(a >= b),
                "==" => RuntimeValue::Bool(a == b),
                "!=" => RuntimeValue::Bool(a != b),
                
                _ => RuntimeValue::Null,
            },
            (RuntimeValue::String(a), RuntimeValue::String(b)) => {
                if op == "+" { 
                    RuntimeValue::String(format!("{}{}", a, b)) 
                } else if op == "==" {
                    RuntimeValue::Bool(a == b)
                } else if op == "!=" {
                    RuntimeValue::Bool(a != b)
                } else { 
                    RuntimeValue::Null 
                }
            },
            _ => {
                error!("Invalid operands for operator {}", op);
                RuntimeValue::Null
            }
        }
    }
}