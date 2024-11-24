use super::Library;
use crate::error::Error;
use crate::parser::Value;
use std::collections::HashMap;

pub struct StdLib {
    functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>>,
    constants: HashMap<String, Value>,
}

impl Library for StdLib {
    fn get_function(&self, name: &str) -> Option<&Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>> {
        self.functions.get(name)
    }

    fn get_constant(&self, name: &str) -> Option<&Value> {
        self.constants.get(name)
    }
}

impl StdLib {
    pub fn new() -> Self {
        let mut lib = StdLib {
            functions: HashMap::new(),
            constants: HashMap::new(),
        };
        
        lib.register_functions();
        lib
    }

    fn register_functions(&mut self) {
        // print() function
        self.functions.insert("print".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("print() takes exactly 1 argument".to_string()));
            }
            println!("{}", args[0]);
            Ok(Value::Null)
        }));

        // len() function
        self.functions.insert("len".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("len() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(s) => Ok(Value::Number(s.chars().count() as i32)),
                Value::Array(arr) => Ok(Value::Number(arr.len() as i32)),
                _ => Err(Error::TypeError(format!(
                    "len() requires string or array argument, got {}", 
                    type_str_of_value(&args[0])
                )))
            }
        }));

        // del() function
        self.functions.insert("del".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("del() takes exactly 1 argument".to_string()));
            }
            // Actual deletion happens in interpreter
            Ok(Value::Null)
        }));

        // type() function
        self.functions.insert("type".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("type() takes exactly 1 argument".to_string()));
            }
            Ok(Value::Type(type_str_of_value(&args[0]).to_string()))
        }));

        // input() function
        self.functions.insert("input".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("input() takes exactly 1 argument".to_string()));
            }
            use std::io::{self, Write};
            
            print!("{}", args[0]);
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            Ok(Value::String(input.trim().to_string()))
        }));
            self.functions.insert("upper".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("upper() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.to_uppercase())),
                    _ => Err(Error::TypeError("upper() requires string argument".to_string()))
                }
            }));
        
            self.functions.insert("lower".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("lower() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.to_lowercase())),
                    _ => Err(Error::TypeError("lower() requires string argument".to_string()))
                }
            }));
        
            self.functions.insert("strip".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("strip() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.trim().to_string())),
                    _ => Err(Error::TypeError("strip() requires string argument".to_string()))
                }
            }));
        
            // Array operations that return values
            self.functions.insert("copy".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("copy() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Array(arr) => Ok(Value::Array(arr.clone())),
                    _ => Err(Error::TypeError("copy() requires array argument".to_string()))
                }
            }));
        
            self.functions.insert("extend".to_string(), Box::new(|args| {
                if args.len() != 2 {
                    return Err(Error::TypeError("extend() takes exactly 2 arguments".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::Array(arr1), Value::Array(arr2)) => {
                        let mut new_arr = arr1.clone();
                        new_arr.extend(arr2.clone());
                        Ok(Value::Array(new_arr))
                    },
                    _ => Err(Error::TypeError("extend() requires two array arguments".to_string()))
                }
            }));
        
            self.functions.insert("insert".to_string(), Box::new(|args| {
                if args.len() < 2 || args.len() > 3 {
                    return Err(Error::TypeError("insert() takes 2 or 3 arguments".to_string()));
                }
                
                match &args[0] {
                    Value::Array(arr) => {
                        let mut new_arr = arr.clone();
                        let value = args[1].clone();
                        
                        if args.len() == 3 {
                            if let Value::Number(index) = args[2] {
                                if index < 0 || index > new_arr.len() as i32 {
                                    return Err(Error::IndexOutOfBounds("Insert index out of bounds".to_string()));
                                }
                                new_arr.insert(index as usize, value);
                            } else {
                                return Err(Error::TypeError("Index must be a number".to_string()));
                            }
                        } else {
                            new_arr.push(value);
                        }
                        Ok(Value::Array(new_arr))
                    },
                    _ => Err(Error::TypeError("First argument must be an array".to_string()))
                }
            }));
        
            self.functions.insert("sort".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("sort() takes exactly 1 argument".to_string()));
                }
                match args[0].clone() {
                    Value::Array(mut arr) => {
                        arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        Ok(Value::Array(arr))
                    },
                    _ => Err(Error::TypeError("sort() requires array argument".to_string()))
                }
            }));
        
            self.functions.insert("reverse".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("reverse() takes exactly 1 argument".to_string()));
                }
                match args[0].clone() {
                    Value::Array(mut arr) => {
                        arr.reverse();
                        Ok(Value::Array(arr))
                    },
                    _ => Err(Error::TypeError("reverse() requires array argument".to_string()))
                }
            }));
        
            self.functions.insert("clear".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("clear() takes exactly 1 argument".to_string()));
                }
                match args[0].clone() {
                    Value::Array(_) => Ok(Value::Array(vec![])),
                    _ => Err(Error::TypeError("clear() requires array argument".to_string()))
                }
            }));
        
            self.functions.insert("count".to_string(), Box::new(|args| {
                if args.len() != 2 {
                    return Err(Error::TypeError("count() takes exactly 2 arguments".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::Array(arr), value) => {
                        Ok(Value::Number(arr.iter().filter(|x| *x == value).count() as i32))
                    },
                    (Value::String(s), Value::String(substr)) => {
                        Ok(Value::Number(s.matches(substr).count() as i32))
                    },
                    _ => Err(Error::TypeError("count() requires (array, value) or (string, string) arguments".to_string()))
                }
            }));
        }
    }

fn type_str_of_value(value: &Value) -> &'static str {
    match value {
        Value::Number(_) => "int",
        Value::String(_) => "str",
        Value::Boolean(_) => "bool",
        Value::Float(_) => "float",
        Value::Null => "null",
        Value::Type(_) => "type",
        Value::Break => "break",
        Value::Continue => "continue",
        Value::Array(_) => "array",
        Value::Function(_, _, _) => "function",
        Value::ReturnValue(val) => type_str_of_value(val),
    }
}