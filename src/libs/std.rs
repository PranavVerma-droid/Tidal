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