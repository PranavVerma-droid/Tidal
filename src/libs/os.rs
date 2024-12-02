use super::Library;
use crate::error::Error;
use crate::parser::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::env;

pub struct OSLib {
    functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>>,
    constants: HashMap<String, Value>,
}

impl Library for OSLib {
    fn get_function(&self, name: &str) -> Option<&Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>> {
        self.functions.get(name)
    }

    fn get_constant(&self, name: &str) -> Option<&Value> {
        self.constants.get(name)
    }

    fn box_clone(&self) -> Box<dyn Library> {
        let mut new_lib = OSLib::new();
        new_lib.constants = self.constants.clone();
        Box::new(new_lib)
    }
}

impl OSLib {
    pub fn new() -> Self {
        let mut lib = OSLib {
            functions: HashMap::new(),
            constants: HashMap::new(),
        };
        lib.register_functions();
        lib.register_constants();
        lib
    }

    fn register_constants(&mut self) {
        self.constants.insert("name".to_string(), Value::String(
            if cfg!(target_os = "windows") { "nt" }
            else { "posix" }.to_string()
        ));
        
        self.constants.insert("linesep".to_string(), Value::String(
            if cfg!(target_os = "windows") { "\r\n" }
            else { "\n" }.to_string()
        ));
    }

    fn register_functions(&mut self) {
        self.functions.insert("makedirs".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("makedirs() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(path) => {
                    let abs_path = std::env::current_dir()
                        .map_err(|e| Error::InterpreterError(e.to_string()))?
                        .join(path);
                    
                    fs::create_dir_all(&abs_path)
                        .map_err(|e| Error::InterpreterError(e.to_string()))?;
                    
                    if !abs_path.is_dir() {
                        return Err(Error::InterpreterError(format!("Failed to create directory: {}", path)));
                    }
                    
                    Ok(Value::Null)
                }
                _ => Err(Error::TypeError("makedirs() requires string argument".to_string()))
            }
        }));

        self.functions.insert("system".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("system() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(cmd) => {
                    let current_dir = std::env::current_dir()
                        .map_err(|e| Error::InterpreterError(e.to_string()))?;

                    #[cfg(target_os = "windows")]
                    let status = Command::new("cmd")
                        .args(&["/C", cmd])
                        .current_dir(current_dir)
                        .status();

                    #[cfg(not(target_os = "windows"))]
                    let status = Command::new("sh")
                        .args(&["-c", cmd])
                        .current_dir(current_dir)
                        .status();

                    match status {
                        Ok(exit_status) => Ok(Value::Number(exit_status.code().unwrap_or(-1))),
                        Err(e) => Err(Error::InterpreterError(e.to_string()))
                    }
                }
                _ => Err(Error::TypeError("system() requires string argument".to_string()))
            }
        }));

        self.functions.insert("rename".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("rename() takes exactly 2 arguments".to_string()));
            }
            match (&args[0], &args[1]) {
                (Value::String(src), Value::String(dst)) => {
                    fs::rename(src, dst)
                        .map_err(|e| Error::InterpreterError(e.to_string()))?;
                    Ok(Value::Null)
                }
                _ => Err(Error::TypeError("rename() requires string arguments".to_string()))
            }
        }));

        self.functions.insert("remove".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("remove() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(path) => {
                    fs::remove_file(path)
                        .map_err(|e| Error::InterpreterError(e.to_string()))?;
                    Ok(Value::Null)
                }
                _ => Err(Error::TypeError("remove() requires string argument".to_string()))
            }
        }));

        self.functions.insert("listdir".to_string(), Box::new(|args| {
            if args.len() > 1 {
                return Err(Error::TypeError("listdir() takes at most 1 argument".to_string()));
            }
            let path = if args.is_empty() {
                "."
            } else if let Value::String(p) = &args[0] {
                p
            } else {
                return Err(Error::TypeError("listdir() requires string argument".to_string()));
            };

            let entries = fs::read_dir(path)
                .map_err(|e| Error::InterpreterError(e.to_string()))?;
            
            let files: Vec<Value> = entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| 
                        e.file_name().into_string().ok().map(Value::String)
                    )
                })
                .collect();

            Ok(Value::Array(files))
        }));

        self.functions.insert("chdir".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("chdir() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(path) => {
                    env::set_current_dir(path)
                        .map_err(|e| Error::InterpreterError(e.to_string()))?;
                    Ok(Value::Null)
                }
                _ => Err(Error::TypeError("chdir() requires string argument".to_string()))
            }
        }));

        self.functions.insert("exists".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("exists() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(path) => {
                    Ok(Value::Boolean(Path::new(path).exists()))
                }
                _ => Err(Error::TypeError("exists() requires string argument".to_string()))
            }
        }));

        self.functions.insert("isfile".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("isfile() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(path) => {
                    Ok(Value::Boolean(Path::new(path).is_file()))
                }
                _ => Err(Error::TypeError("isfile() requires string argument".to_string()))
            }
        }));

        self.functions.insert("isdir".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("isdir() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(path) => {
                    Ok(Value::Boolean(Path::new(path).is_dir()))
                }
                _ => Err(Error::TypeError("isdir() requires string argument".to_string()))
            }
        }));

        self.functions.insert("system".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("system() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(cmd) => {
                    #[cfg(target_os = "windows")]
                    let status = Command::new("cmd")
                        .args(&["/C", cmd])
                        .status();

                    #[cfg(not(target_os = "windows"))]
                    let status = Command::new("sh")
                        .args(&["-c", cmd])
                        .status();

                    match status {
                        Ok(exit_status) => Ok(Value::Number(exit_status.code().unwrap_or(-1))),
                        Err(e) => Err(Error::InterpreterError(e.to_string()))
                    }
                }
                _ => Err(Error::TypeError("system() requires string argument".to_string()))
            }
        }));

        self.functions.insert("removedirs".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("removedirs() takes exactly 1 argument".to_string()));
            }
            match &args[0] {
                Value::String(path) => {
                    let path = Path::new(path);
                    

                    fn remove_parents(path: &Path) -> std::io::Result<()> {
                        if let Some(parent) = path.parent() {
                            if parent.as_os_str().is_empty() {
                                return Ok(());
                            }
                            
                    
                            match fs::remove_dir(parent) {
                                Ok(_) => remove_parents(parent), 
                                Err(_) => Ok(()) 
                            }
                        } else {
                            Ok(())
                        }
                    }
        

                    fs::remove_dir_all(path)
                        .map_err(|e| Error::InterpreterError(e.to_string()))?;
        
                    remove_parents(path)
                        .map_err(|e| Error::InterpreterError(e.to_string()))?;
        
                    Ok(Value::Null)
                }
                _ => Err(Error::TypeError("removedirs() requires string argument".to_string()))
            }
        }));
    }
}