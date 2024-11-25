pub mod std;
pub mod math;
pub mod sys;
pub mod os;

use crate::error::Error;
use crate::parser::Value;

pub trait Library {
    fn get_function(&self, name: &str) -> Option<&Box<dyn Fn(Vec<Value>) -> Result<Value, Error>>>;
    fn get_constant(&self, name: &str) -> Option<&Value>;
}