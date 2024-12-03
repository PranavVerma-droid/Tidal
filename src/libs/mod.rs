pub mod std;
pub mod math;
pub mod sys;
pub mod os;
pub mod io;

use crate::error::Error;
use crate::parser::Value;

#[allow(dead_code)]
pub trait Library {
    fn get_function(&self, name: &str) -> Option<&Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>>;
    fn get_constant(&self, name: &str) -> Option<&Value>;
    fn is_mutable(&self, name: &str) -> Option<bool>;
    fn box_clone(&self) -> Box<dyn Library>;
}