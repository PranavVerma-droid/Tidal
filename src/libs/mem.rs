use super::Library;
use crate::error::Error;
use crate::parser::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::mem;

pub struct MemLib {
    functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>>,
    constants: HashMap<String, Value>,
    var_mutability: HashMap<String, bool>,
}

impl Library for MemLib {
    fn get_function(&self, name: &str) -> Option<&Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>> {
        self.functions.get(name)
    }

    fn get_constant(&self, name: &str) -> Option<&Value> {
        self.constants.get(name)
    }

    fn box_clone(&self) -> Box<dyn Library> {
        let mut new_lib = MemLib::new();
        new_lib.constants = self.constants.clone();
        new_lib.var_mutability = self.var_mutability.clone();
        Box::new(new_lib)
    }

    fn is_mutable(&self, name: &str) -> Option<bool> {
        self.var_mutability.get(name).copied()
    }
}

impl MemLib {
    pub fn new() -> Self {
        let mut lib = MemLib {
            functions: HashMap::new(),
            constants: HashMap::new(),
            var_mutability: HashMap::new(),
        };
        lib.register_functions();
        lib.register_constants();
        lib
    }

    fn register_constants(&mut self) {
        self.constants.insert("POINTER_SIZE".to_string(), Value::Number(mem::size_of::<usize>() as i32));
        self.constants.insert("MAX_INT".to_string(), Value::Number(i32::MAX));
        self.constants.insert("MIN_INT".to_string(), Value::Number(i32::MIN));
        self.constants.insert("ALIGNMENT_MAX".to_string(), Value::Number(mem::align_of::<usize>() as i32));
        self.constants.insert("WORD_SIZE".to_string(), Value::Number(mem::size_of::<usize>() as i32)); 
        self.constants.insert("CACHE_LINE".to_string(), Value::Number(64));
        self.constants.insert("PAGE_SIZE".to_string(), Value::Number(4096));
    }

    fn register_functions(&mut self) {
        self.functions.insert("sizeof".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("sizeof() takes exactly 1 argument".to_string()));
            }
            
            let size = match &args[0] {
                Value::Number(_) => mem::size_of::<i32>(),
                Value::Float(_) => mem::size_of::<f64>(),
                Value::Boolean(_) => mem::size_of::<bool>(),
                Value::String(s) => s.capacity(),
                Value::Array(arr) => {
                    let guard = arr.lock().unwrap();
                    guard.capacity() * mem::size_of::<Value>()
                },
                Value::Function(_, _, _) => mem::size_of::<Value>(),
                _ => mem::size_of::<Value>(),
            };
            
            Ok(Value::Number(size as i32))
        }));

        self.functions.insert("deepcopy".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("deepcopy() takes exactly 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::Array(arr) => {
                    let guard = arr.lock().unwrap();
                    let mut new_arr = Vec::new();
                    
                    for item in guard.iter() {
                        new_arr.push(match item {
                            Value::Array(nested_arr) => {
                                let nested_guard = nested_arr.lock().unwrap();
                                Value::Array(Arc::new(Mutex::new(nested_guard.clone())))
                            },
                            _ => item.clone(),
                        });
                    }
                    
                    Ok(Value::Array(Arc::new(Mutex::new(new_arr))))
                },
                _ => Ok(args[0].clone()),
            }
        }));

        self.functions.insert("getrefcount".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("getrefcount() takes exactly 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::Array(arr) => {
                    let count = Arc::strong_count(arr);
                    Ok(Value::Number(count as i32))
                },
                _ => Ok(Value::Number(1)),
            }
        }));

        self.functions.insert("allocated".to_string(), Box::new(|args| {
            if !args.is_empty() {
                return Err(Error::TypeError("allocated() takes no arguments".to_string()));
            }
            
            let stats = vec![
                Value::Number(0),
                Value::Number(0),
            ];
            
            Ok(Value::Array(Arc::new(Mutex::new(stats))))
        }));

        self.functions.insert("id".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("id() takes exactly 1 argument".to_string()));
            }
            
            let addr = match &args[0] {
                Value::Array(arr) => {
                    let ptr = Arc::as_ptr(arr) as usize;
                    ptr as i32
                },
                _ => {
                    let ptr = &args[0] as *const Value as usize;
                    ptr as i32
                }
            };
            
            Ok(Value::Number(addr))
        }));

        self.functions.insert("getsizeof".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("getsizeof() takes exactly 1 argument".to_string()));
            }
            
            let size = match &args[0] {
                Value::String(s) => s.capacity() + mem::size_of::<String>(),
                Value::Array(arr) => {
                    let guard = arr.lock().unwrap();
                    guard.capacity() * mem::size_of::<Value>() + mem::size_of::<Vec<Value>>()
                },
                _ => mem::size_of::<Value>(),
            };
            
            Ok(Value::Number(size as i32))
        }));

        self.functions.insert("is".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("is() takes exactly 2 arguments".to_string()));
            }
            
            let is_same = match (&args[0], &args[1]) {
                (Value::Array(arr1), Value::Array(arr2)) => {
                    Arc::ptr_eq(arr1, arr2)
                },
                _ => std::ptr::eq(&args[0], &args[1]),
            };
            
            Ok(Value::Boolean(is_same))
        }));

        self.functions.insert("getalign".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("getalign() takes exactly 1 argument".to_string()));
            }
            
            let alignment = match &args[0] {
                Value::Number(_) => mem::align_of::<i32>(),
                Value::Float(_) => mem::align_of::<f64>(),
                Value::Boolean(_) => mem::align_of::<bool>(),
                Value::String(_) => mem::align_of::<String>(),
                Value::Array(_) => mem::align_of::<Vec<Value>>(),
                _ => mem::align_of::<Value>(),
            };
            
            Ok(Value::Number(alignment as i32))
        }));

        self.functions.insert("isshared".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("isshared() takes exactly 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::Array(arr) => {
                    let count = Arc::strong_count(arr);
                    Ok(Value::Boolean(count > 1))
                },
                _ => Ok(Value::Boolean(false)),
            }
        }));

        self.functions.insert("memdiff".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("memdiff() takes exactly 2 arguments".to_string()));
            }
            
            let addr1 = &args[0] as *const Value as usize;
            let addr2 = &args[1] as *const Value as usize;
            let diff = if addr1 > addr2 { addr1 - addr2 } else { addr2 - addr1 };
            
            Ok(Value::Number(diff as i32))
        }));

        self.functions.insert("meminfo".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("meminfo() takes exactly 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::Array(arr) => {
                    let guard = arr.lock().unwrap();
                    let info = vec![
                        Value::Number(guard.len() as i32),        // length
                        Value::Number(guard.capacity() as i32),   // capacity
                        Value::Number((guard.capacity() * mem::size_of::<Value>()) as i32), // total bytes
                        Value::Number(Arc::strong_count(arr) as i32), // reference count
                    ];
                    Ok(Value::Array(Arc::new(Mutex::new(info))))
                },
                _ => Err(Error::TypeError("meminfo() requires array argument".to_string()))
            }
        }));

        self.functions.insert("fraginfo".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("fraginfo() takes exactly 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::Array(arr) => {
                    let guard = arr.lock().unwrap();
                    let capacity = guard.capacity();
                    let length = guard.len();
                    let wasted = capacity - length;
                    let frag_percent = if capacity > 0 {
                        (wasted as f64 / capacity as f64) * 100.0
                    } else {
                        0.0
                    };
                    
                    let info = vec![
                        Value::Number(wasted as i32),      // wasted slots
                        Value::Float(frag_percent),        // fragmentation percentage
                    ];
                    Ok(Value::Array(Arc::new(Mutex::new(info))))
                },
                _ => Err(Error::TypeError("fraginfo() requires array argument".to_string()))
            }
        }));

        self.functions.insert("shrink".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("shrink() takes exactly 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::Array(arr) => {
                    let mut guard = arr.lock().unwrap();
                    let mut new_vec = Vec::new();
                    new_vec.extend(guard.iter().cloned());
                    new_vec.shrink_to_fit();
                    *guard = new_vec;
                    Ok(Value::Null)
                },
                _ => Err(Error::TypeError("shrink() requires array argument".to_string()))
            }
        }));

        self.functions.insert("reserve".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("reserve() takes exactly 2 arguments".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::Array(arr), Value::Number(additional)) => {
                    let mut guard = arr.lock().unwrap();
                    guard.reserve(*additional as usize);
                    Ok(Value::Null)
                },
                _ => Err(Error::TypeError("reserve() requires array and number arguments".to_string()))
            }
        }));

        self.functions.insert("sharemem".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("sharemem() takes exactly 2 arguments".to_string()));
            }
            
            match (&args[0], &args[1]) {
                (Value::Array(arr1), Value::Array(arr2)) => {
                    let ptr1 = Arc::as_ptr(arr1);
                    let ptr2 = Arc::as_ptr(arr2);
                    Ok(Value::Boolean(ptr1 == ptr2))
                },
                _ => Err(Error::TypeError("sharemem() requires two array arguments".to_string()))
            }
        }));

        self.functions.insert("memrange".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("memrange() takes exactly 1 argument".to_string()));
            }
            
            match &args[0] {
                Value::Array(arr) => {
                    let guard = arr.lock().unwrap();
                    let start = guard.as_ptr() as usize;
                    let end = start + (guard.len() * mem::size_of::<Value>());
                    
                    Ok(Value::Array(Arc::new(Mutex::new(vec![
                        Value::Number(start as i32),
                        Value::Number(end as i32),
                    ]))))
                },
                _ => Err(Error::TypeError("memrange() requires array argument".to_string()))
            }
        }));
    }
}
