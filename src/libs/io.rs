use std::fs::{self, OpenOptions};
use std::path::{PathBuf, MAIN_SEPARATOR};
use std::collections::HashMap;
use std::env;
use std::io::{Write};

use crate::error::Error;
use crate::parser::Value;
use super::Library;

pub struct IOLib {
    functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>>,
    constants: HashMap<String, Value>,
}

impl Library for IOLib {
    fn get_function(&self, name: &str) -> Option<&Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>> {
        self.functions.get(name)
    }

    fn get_constant(&self, name: &str) -> Option<&Value> {
        self.constants.get(name)
    }

    fn is_mutable(&self, _name: &str) -> Option<bool> {
        None
    }

    fn box_clone(&self) -> Box<dyn Library> {
        let mut new_lib = IOLib::new();
        new_lib.constants = self.constants.clone();
        Box::new(new_lib)
    }
}

impl IOLib {
    fn normalize_path(path: &str) -> String {
        path.replace('\\', "/")
            .replace('/', &MAIN_SEPARATOR.to_string())
    }

    fn get_absolute_path(path: &str) -> Result<PathBuf, Error> {
        let normalized = Self::normalize_path(path);
        let path_buf = PathBuf::from(&normalized);
        
        if path_buf.is_absolute() {
            Ok(path_buf)
        } else {
            env::current_dir()
                .map_err(|e| Error::FileNotFound(format!("Failed to get current directory: {}", e)))
                .map(|dir| dir.join(normalized))
        }
    }

    pub fn new() -> Self {
        let mut lib = IOLib {
            functions: HashMap::new(),
            constants: HashMap::new(),
        };

        lib.functions.insert("open".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("open() takes exactly 2 arguments".to_string()));
            }

            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::TypeError("Filename must be a string".to_string())),
            };

            let mode = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::TypeError("Mode must be a string".to_string())),
            };

            let abs_path = IOLib::get_absolute_path(&path)?;

            if let Some(parent) = abs_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| 
                        Error::FileNotFound(format!("Failed to create directories: {}", e))
                    )?;
                }
            }

            let mut options = OpenOptions::new();
            match mode.as_str() {
                "r" => { options.read(true); }
                "w" => { options.write(true).create(true).truncate(true); }
                "w+" => { options.read(true).write(true).create(true).truncate(true); }
                "a" => { options.append(true).create(true); }
                "a+" => { options.read(true).append(true).create(true); }
                _ => return Err(Error::TypeError("Invalid file mode. Use: r, w, w+, a, or a+".to_string())),
            };

            options.open(&abs_path)
                .map_err(|e| Error::FileNotFound(format!("Failed to open file: {}", e)))?;

            Ok(Value::String(abs_path.to_string_lossy().into_owned()))
        }));

        lib.functions.insert("write".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("write() takes exactly 2 arguments".to_string()));
            }

            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::TypeError("Filename must be a string".to_string())),
            };

            let content = match &args[1] {
                Value::String(s) => s.clone(),
                _ => format!("{}", args[1]),
            };

            let abs_path = IOLib::get_absolute_path(&path)?;

            if let Some(parent) = abs_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| 
                        Error::FileNotFound(format!("Failed to create directories: {}", e))
                    )?;
                }
            }

            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&abs_path)
                .and_then(|mut file| file.write_all(content.as_bytes()))
                .map_err(|e| Error::FileNotFound(format!("Failed to write to file: {}", e)))?;

            Ok(Value::Null)
        }));

        lib.functions.insert("read".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("read() takes exactly 1 argument".to_string()));
            }

            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::TypeError("Filename must be a string".to_string())),
            };

            let abs_path = IOLib::get_absolute_path(&path)?;

            if !abs_path.exists() {
                return Err(Error::FileNotFound(format!("File does not exist: {}", abs_path.display())));
            }

            fs::read_to_string(&abs_path)
                .map(Value::String)
                .map_err(|e| Error::FileNotFound(format!("Failed to read file: {}", e)))
        }));

        lib.functions.insert("append".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("append() takes exactly 2 arguments".to_string()));
            }

            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::TypeError("Filename must be a string".to_string())),
            };

            let content = match &args[1] {
                Value::String(s) => s.clone(),
                _ => format!("{}", args[1]),
            };

            let abs_path = IOLib::get_absolute_path(&path)?;

            if let Some(parent) = abs_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| 
                        Error::FileNotFound(format!("Failed to create directories: {}", e))
                    )?;
                }
            }

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&abs_path)
                .map_err(|e| Error::FileNotFound(format!("Failed to open file: {}", e)))?;

            file.write_all(content.as_bytes())
                .map_err(|e| Error::FileNotFound(format!("Failed to append to file: {}", e)))?;

            Ok(Value::Null)
        }));

        lib.functions.insert("exists".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("exists() takes exactly 1 argument".to_string()));
            }

            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::TypeError("Path must be a string".to_string())),
            };

            let abs_path = IOLib::get_absolute_path(&path)?;
            Ok(Value::Boolean(abs_path.exists()))
        }));

        lib.functions.insert("remove".to_string(), Box::new(|args| {
            if args.len() != 1 {
                return Err(Error::TypeError("remove() takes exactly 1 argument".to_string()));
            }

            let path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::TypeError("Path must be a string".to_string())),
            };

            let abs_path = IOLib::get_absolute_path(&path)?;

            if !abs_path.exists() {
                return Ok(Value::Null);
            }

            fs::remove_file(&abs_path)
                .map_err(|e| Error::FileNotFound(format!("Failed to remove file: {}", e)))?;

            Ok(Value::Null)
        }));

        lib.functions.insert("rename".to_string(), Box::new(|args| {
            if args.len() != 2 {
                return Err(Error::TypeError("rename() takes exactly 2 arguments".to_string()));
            }

            let old_path = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::TypeError("Old path must be a string".to_string())),
            };

            let new_path = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::TypeError("New path must be a string".to_string())),
            };

            let abs_old_path = IOLib::get_absolute_path(&old_path)?;
            let abs_new_path = IOLib::get_absolute_path(&new_path)?;

            if !abs_old_path.exists() {
                return Err(Error::FileNotFound(format!("Source file does not exist: {}", abs_old_path.display())));
            }

            if let Some(parent) = abs_new_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| 
                        Error::FileNotFound(format!("Failed to create directories: {}", e))
                    )?;
                }
            }

            fs::rename(&abs_old_path, &abs_new_path)
                .map_err(|e| Error::FileNotFound(format!("Failed to rename file: {}", e)))?;

            Ok(Value::Null)
        }));

        lib
    }
}