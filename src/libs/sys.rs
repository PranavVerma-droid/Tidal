use super::Library;
use crate::error::Error;
use crate::parser::Value;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[cfg(target_family = "unix")]
use sys_info;

pub struct SysLib {
    functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>>,
    constants: HashMap<String, Value>,
}

impl Library for SysLib {
    fn get_function(&self, name: &str) -> Option<&Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>> {
        self.functions.get(name)
    }

    fn get_constant(&self, name: &str) -> Option<&Value> {
        self.constants.get(name)
    }

    fn box_clone(&self) -> Box<dyn Library> {
        // Create a new instance which will recreate all functions
        let mut new_lib = SysLib::new();
        new_lib.constants = self.constants.clone();
        Box::new(new_lib)
    }
}

impl SysLib {
    pub fn new() -> Self {
        let mut lib = SysLib {
            functions: HashMap::new(),
            constants: HashMap::new(),
        };
        lib.register_functions();
        lib.register_constants();
        lib
    }

    fn register_constants(&mut self) {
        // Platform specific constants
        self.constants.insert("PLATFORM".to_string(), Value::String(
            if cfg!(target_os = "windows") { "windows" }
            else if cfg!(target_os = "linux") { "linux" }
            else if cfg!(target_os = "macos") { "darwin" }
            else { "unknown" }.to_string()
        ));

        // Command line arguments
        let args: Vec<Value> = env::args()
            .map(|arg| Value::String(arg))
            .collect();
        self.constants.insert("ARGV".to_string(), Value::Array(Arc::new(Mutex::new(args))));

        // Executable path
        if let Ok(exe_path) = env::current_exe() {
            if let Some(path_str) = exe_path.to_str() {
                self.constants.insert("EXECUTABLE".to_string(), 
                    Value::String(path_str.to_string()));
            }
        }

        // Version info
        self.constants.insert("VERSION".to_string(), 
            Value::String(env!("CARGO_PKG_VERSION").to_string()));

        // Path separator
        self.constants.insert("PATH_SEP".to_string(), 
            Value::String(std::path::MAIN_SEPARATOR.to_string()));

        // Max sizes
        self.constants.insert("MAXSIZE".to_string(), 
            Value::Number(i32::MAX));

        // Platform details
        self.constants.insert("OS_NAME".to_string(), Value::String(
            std::env::consts::OS.to_string()
        ));
        self.constants.insert("ARCH".to_string(), Value::String(
            std::env::consts::ARCH.to_string()
        ));

        // Environment paths
        if let Ok(path) = env::var("PATH") {
            let path_array: Vec<Value> = path.split(':')
                .map(|s| Value::String(s.to_string()))
                .collect();
            self.constants.insert("PATH".to_string(), Value::Array(Arc::new(Mutex::new(path_array))));
        }
    }

    fn register_functions(&mut self) {
        // Existing functions
        self.register_process_functions();
        self.register_env_functions();
        self.register_path_functions();
        self.register_platform_functions();
    }

    fn register_process_functions(&mut self) {
        // exit(code)
        self.functions.insert("exit".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("exit() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::Number(code) => {
                    std::process::exit(*code);
                }
                _ => return Err(Error::TypeError("exit() requires integer argument".to_string()))
            }
        }));

        // getpid()
        self.functions.insert("getpid".to_string(), Box::new(|args| {
            if !args.is_empty() {
                return Err(Error::TypeError("getpid() takes no arguments".to_string()));
            }
            Ok(Value::Number(std::process::id() as i32))
        }));
    }

    fn register_env_functions(&mut self) {
        // getenv(name)
        self.functions.insert("getenv".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("getenv() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(name) => {
                    match env::var(name) {
                        Ok(val) => Ok(Value::String(val)),
                        Err(_) => Ok(Value::Null)
                    }
                }
                _ => Err(Error::TypeError("getenv() requires string argument".to_string()))
            }
        }));

        // setenv(name, value)
        self.functions.insert("setenv".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("setenv() takes exactly 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(name), Value::String(value)) => {
                    env::set_var(name, value);
                    Ok(Value::Null)
                }
                _ => Err(Error::TypeError("setenv() requires string arguments".to_string()))
            }
        }));

        // unsetenv(name)
        self.functions.insert("unsetenv".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("unsetenv() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(name) => {
                    env::remove_var(name);
                    Ok(Value::Null)
                }
                _ => Err(Error::TypeError("unsetenv() requires string argument".to_string()))
            }
        }));
    }

    fn register_path_functions(&mut self) {
        // getcwd()
        self.functions.insert("getcwd".to_string(), Box::new(|args| {
            if !args.is_empty() {
                return Err(Error::TypeError("getcwd() takes no arguments".to_string()));
            }
            match env::current_dir() {
                Ok(path) => {
                    if let Some(path_str) = path.to_str() {
                        Ok(Value::String(path_str.to_string()))
                    } else {
                        Ok(Value::Null)
                    }
                }
                Err(_) => Ok(Value::Null)
            }
        }));

        // abspath(path)
        self.functions.insert("abspath".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("abspath() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(path) => {
                    let path_buf = PathBuf::from(path);
                    match path_buf.canonicalize() {
                        Ok(abs_path) => {
                            if let Some(path_str) = abs_path.to_str() {
                                Ok(Value::String(path_str.to_string()))
                            } else {
                                Ok(Value::Null)
                            }
                        }
                        Err(_) => Ok(Value::Null)
                    }
                }
                _ => Err(Error::TypeError("abspath() requires string argument".to_string()))
            }
        }));
    }

    fn register_platform_functions(&mut self) {
        //for unix only
        self.functions.insert("getloadavg".to_string(), Box::new(|args| {
            if !args.is_empty() {
                return Err(Error::TypeError("getloadavg() takes no arguments".to_string()));
            }
            
            #[cfg(target_family = "unix")]
            {
                match sys_info::loadavg() {
                    Ok(loads) => {
                        return Ok(Value::Array(Arc::new(Mutex::new(vec![
                            Value::Float(loads.one),
                            Value::Float(loads.five),
                            Value::Float(loads.fifteen),
                        ]))));
                    }
                    Err(_) => return Ok(Value::Null)
                }
            }
            
            #[cfg(not(target_family = "unix"))]
            {
                Ok(Value::Null)
            }
        }));

        // getsizeof(obj)
        self.functions.insert("getsizeof".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("getsizeof() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::Array(arr) => {
                    let guard = arr.lock().unwrap();
                    Ok(Value::Number(guard.len() as i32))
                },
                Value::String(s) => Ok(Value::Number(s.len() as i32)),
                _ => Ok(Value::Number(std::mem::size_of::<Value>() as i32))
            }
        }));
    }
}