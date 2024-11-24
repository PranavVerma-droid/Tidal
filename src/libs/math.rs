use super::Library;
use crate::error::Error;
use crate::parser::Value;
use std::collections::HashMap;

pub struct MathLib {
    functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>>,
    constants: HashMap<String, Value>,
}

impl Library for MathLib {
    fn get_function(&self, name: &str) -> Option<&Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>> {
        self.functions.get(name)
    }

    fn get_constant(&self, name: &str) -> Option<&Value> {
        self.constants.get(name)
    }
}

impl MathLib {
    pub fn new() -> Self {
        let mut lib = MathLib {
            functions: HashMap::new(),
            constants: HashMap::new(),
        };
        
        lib.register_functions();
        lib.register_constants();
        lib
    }

    
        fn register_constants(&mut self) {
            self.constants.insert("PI".to_string(), Value::Float(std::f64::consts::PI));
            self.constants.insert("E".to_string(), Value::Float(std::f64::consts::E));
            self.constants.insert("TAU".to_string(), Value::Float(std::f64::consts::TAU));
            self.constants.insert("INF".to_string(), Value::Float(f64::INFINITY));
        }
    
        fn register_functions(&mut self) {
            self.functions.insert("abs".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("abs() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(n.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => Err(Error::TypeError("abs() requires numeric argument".to_string()))
                }
            }));
    
            self.functions.insert("pow".to_string(), Box::new(|args| {
                if args.len() != 2 {
                    return Err(Error::TypeError("pow() takes exactly 2 arguments".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::Number(base), Value::Number(exp)) => 
                        Ok(Value::Float((*base as f64).powf(*exp as f64))),
                    (Value::Float(base), Value::Number(exp)) => 
                        Ok(Value::Float(base.powf(*exp as f64))),
                    (Value::Number(base), Value::Float(exp)) => 
                        Ok(Value::Float((*base as f64).powf(*exp))),
                    (Value::Float(base), Value::Float(exp)) => 
                        Ok(Value::Float(base.powf(*exp))),
                    _ => Err(Error::TypeError("pow() requires numeric arguments".to_string()))
                }
            }));
            
            self.functions.insert("gcd".to_string(), Box::new(|args| {
                if args.len() != 2 {
                    return Err(Error::TypeError("gcd() takes exactly 2 arguments".to_string()));
                }

                fn calculate_gcd(mut a: i32, mut b: i32) -> i32 {
                    a = a.abs();
                    b = b.abs();
                    while b != 0 {
                        let temp = b;
                        b = a % b;
                        a = temp;
                    }
                    a
                }

                match (&args[0], &args[1]) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(calculate_gcd(*a, *b))),
                    _ => Err(Error::TypeError("gcd() requires integer arguments".to_string()))
                }
            }));
    
            self.functions.insert("sqrt".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("sqrt() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Float((*n as f64).sqrt())),
                    Value::Float(f) => Ok(Value::Float(f.sqrt())),
                    _ => Err(Error::TypeError("sqrt() requires numeric argument".to_string()))
                }
            }));
    

            self.functions.insert("sin".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("sin() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Float((*n as f64).sin())),
                    Value::Float(f) => Ok(Value::Float(f.sin())),
                    _ => Err(Error::TypeError("sin() requires numeric argument".to_string()))
                }
            }));
    
            self.functions.insert("cos".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("cos() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Float((*n as f64).cos())),
                    Value::Float(f) => Ok(Value::Float(f.cos())),
                    _ => Err(Error::TypeError("cos() requires numeric argument".to_string()))
                }
            }));
    
            self.functions.insert("tan".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("tan() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Float((*n as f64).tan())),
                    Value::Float(f) => Ok(Value::Float(f.tan())),
                    _ => Err(Error::TypeError("tan() requires numeric argument".to_string()))
                }
            }));
    
            // Logarithmic functions
            self.functions.insert("log".to_string(), Box::new(|args| {
                if args.len() != 2 {
                    return Err(Error::TypeError("log() takes exactly 2 arguments".to_string()));
                }
                match (&args[0], &args[1]) {
                    (Value::Number(n), Value::Number(base)) => 
                        Ok(Value::Float((*n as f64).log(*base as f64))),
                    (Value::Float(n), Value::Number(base)) => 
                        Ok(Value::Float(n.log(*base as f64))),
                    (Value::Number(n), Value::Float(base)) => 
                        Ok(Value::Float((*n as f64).log(*base))),
                    (Value::Float(n), Value::Float(base)) => 
                        Ok(Value::Float(n.log(*base))),
                    _ => Err(Error::TypeError("log() requires numeric arguments".to_string()))
                }
            }));
    
            self.functions.insert("ln".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("ln() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Float((*n as f64).ln())),
                    Value::Float(f) => Ok(Value::Float(f.ln())),
                    _ => Err(Error::TypeError("ln() requires numeric argument".to_string()))
                }
            }));
    
            // Rounding functions
            self.functions.insert("ceil".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("ceil() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(*n)),
                    Value::Float(f) => Ok(Value::Number(f.ceil() as i32)),
                    _ => Err(Error::TypeError("ceil() requires numeric argument".to_string()))
                }
            }));
    
            self.functions.insert("floor".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("floor() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(*n)),
                    Value::Float(f) => Ok(Value::Number(f.floor() as i32)),
                    _ => Err(Error::TypeError("floor() requires numeric argument".to_string()))
                }
            }));
    
            self.functions.insert("round".to_string(), Box::new(|args| {
                if args.len() != 1 {
                    return Err(Error::TypeError("round() takes exactly 1 argument".to_string()));
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(*n)),
                    Value::Float(f) => Ok(Value::Number(f.round() as i32)),
                    _ => Err(Error::TypeError("round() requires numeric argument".to_string()))
                }
            }));
        }
    }