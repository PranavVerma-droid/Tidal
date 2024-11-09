use crate::parser::{ASTNode, Value};
use crate::lexer::Token;
use crate::error::Error;
use std::collections::HashMap;
use std::fmt;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Null => write!(f, "null"),
            Value::Type(t) => write!(f, "{}", t),
            Value::Break => write!(f, "break"),
            Value::Continue => write!(f, "continue"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, value) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            },
        }
    }
}

pub fn interpret(ast: Vec<ASTNode>, is_verbose: bool) -> Result<Option<Value>, Error> {
    let mut symbol_table: HashMap<String, (Value, bool)> = HashMap::new(); // (Value, is_mutable)
    let mut result = None;

    for node in ast {
        result = Some(interpret_node(&node, &mut symbol_table, is_verbose, false)?);
        if let Value::Break = result.as_ref().unwrap() {
            return Err(Error::BreakOutsideLoop);
        }
        if let Value::Continue = result.as_ref().unwrap() {
            return Err(Error::ContinueOutsideLoop);
        }
    }

    Ok(result)
}

fn interpret_node(node: &ASTNode, symbol_table: &mut HashMap<String, (Value, bool)>, is_verbose: bool, in_loop: bool) -> Result<Value, Error> {
    match node {
        ASTNode::Number(val) => Ok(Value::Number(*val)),
        ASTNode::String(val) => Ok(Value::String(val.clone())),
        ASTNode::Float(val) => Ok(Value::Float(*val)),
        ASTNode::Boolean(val) => Ok(Value::Boolean(*val)),
        ASTNode::Null => Ok(Value::Null),
        ASTNode::BinaryOp(left, op, right) => {
            let left_val = interpret_node(left, symbol_table, is_verbose, in_loop)?;
            match op {
                Token::And => {
                    if let Value::Boolean(false) = left_val {
                        return Ok(Value::Boolean(false));
                    }
                    let right_val = interpret_node(right, symbol_table, is_verbose, in_loop)?;
                    match (left_val, right_val) {
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l && r)),
                        _ => Err(Error::TypeError(format!("AND operator can only be applied to boolean values"))),
                    }
                },
                Token::Or => {
                    if let Value::Boolean(true) = left_val {
                        return Ok(Value::Boolean(true));
                    }
                    let right_val = interpret_node(right, symbol_table, is_verbose, in_loop)?;
                    match (left_val, right_val) {
                        (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l || r)),
                        _ => Err(Error::TypeError(format!("OR operator can only be applied to boolean values"))),
                    }
                },
                _ => {
                    let right_val = interpret_node(right, symbol_table, is_verbose, in_loop)?;
                    match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => {
                            match op {
                                Token::Plus => Ok(Value::Number(l + r)),
                                Token::Minus => Ok(Value::Number(l - r)),
                                Token::Multiply => Ok(Value::Number(l * r)),
                                Token::Divide => Ok(Value::Float(l as f64 / r as f64)),
                                Token::Equal => Ok(Value::Boolean(l == r)),
                                Token::NotEqual => Ok(Value::Boolean(l != r)),
                                Token::Greater => Ok(Value::Boolean(l > r)),
                                Token::Less => Ok(Value::Boolean(l < r)),
                                Token::GreaterEqual => Ok(Value::Boolean(l >= r)),
                                Token::FloorDivide => Ok(Value::Number(l / r)),
                                Token::LessEqual => Ok(Value::Boolean(l <= r)),
                                Token::Modulus => Ok(Value::Number(l % r)),
                                Token::Power => Ok(Value::Number(l.pow(r as u32))),
                                _ => Err(Error::UnsupportedOperation(format!("Unsupported operator for numbers"))),
                            }
                        }
                        (Value::Float(l), Value::Float(r)) => {
                            match op {
                                Token::Plus => Ok(Value::Float(l + r)),
                                Token::Minus => Ok(Value::Float(l - r)),
                                Token::Multiply => Ok(Value::Float(l * r)),
                                Token::Divide => Ok(Value::Float(l / r)),
                                Token::Equal => Ok(Value::Boolean(l == r)),
                                Token::NotEqual => Ok(Value::Boolean(l != r)),
                                Token::Greater => Ok(Value::Boolean(l > r)),
                                Token::Modulus => Ok(Value::Float(l % r)),
                                Token::FloorDivide => Ok(Value::Number((l / r).floor() as i32)),
                                Token::Less => Ok(Value::Boolean(l < r)),
                                Token::GreaterEqual => Ok(Value::Boolean(l >= r)),
                                Token::LessEqual => Ok(Value::Boolean(l <= r)),
                                Token::Power => Ok(Value::Float(l.powf(r))),
                                _ => Err(Error::UnsupportedOperation(format!("Unsupported operator for floats"))),
                            }
                        }
                        (Value::Number(l), Value::Float(r)) => {
                            let l = l as f64;
                            match op {
                                Token::Plus => Ok(Value::Float(l + r)),
                                Token::Minus => Ok(Value::Float(l - r)),
                                Token::Multiply => Ok(Value::Float(l * r)),
                                Token::Divide => Ok(Value::Float(l / r)),
                                Token::Equal => Ok(Value::Boolean(l == r)),
                                Token::Modulus => Ok(Value::Float(l % r)),
                                Token::NotEqual => Ok(Value::Boolean(l != r)),
                                Token::FloorDivide => Ok(Value::Number((l / r).floor() as i32)),
                                Token::Greater => Ok(Value::Boolean(l > r)),
                                Token::Less => Ok(Value::Boolean(l < r)),
                                Token::GreaterEqual => Ok(Value::Boolean(l >= r)),
                                Token::Power => Ok(Value::Float(l.powf(r))),
                                Token::LessEqual => Ok(Value::Boolean(l <= r)),
                                _ => Err(Error::UnsupportedOperation(format!("Unsupported operator for mixed number and float"))),
                            }
                        }
                        (Value::Float(l), Value::Number(r)) => {
                            let r = r as f64;
                            match op {
                                Token::Plus => Ok(Value::Float(l + r)),
                                Token::Minus => Ok(Value::Float(l - r)),
                                Token::Multiply => Ok(Value::Float(l * r)),
                                Token::Divide => Ok(Value::Float(l / r)),
                                Token::Equal => Ok(Value::Boolean(l == r)),
                                Token::Modulus => Ok(Value::Float(l % r)),
                                Token::NotEqual => Ok(Value::Boolean(l != r)),
                                Token::Greater => Ok(Value::Boolean(l > r)),
                                Token::Less => Ok(Value::Boolean(l < r)),
                                Token::GreaterEqual => Ok(Value::Boolean(l >= r)),
                                Token::FloorDivide => Ok(Value::Number((l / r).floor() as i32)),
                                Token::Power => Ok(Value::Float(l.powf(r))),
                                Token::LessEqual => Ok(Value::Boolean(l <= r)),
                                _ => Err(Error::UnsupportedOperation(format!("Unsupported operator for mixed float and number"))),
                            }
                        }
                        (Value::String(s), Value::String(t)) => {
                            match op {
                                Token::Plus => Ok(Value::String(s + &t)),
                                Token::Multiply => {
                                    if let Value::Number(n) = interpret_node(right, symbol_table, is_verbose, in_loop)? {
                                        Ok(Value::String(s.repeat(n as usize)))
                                    } else {
                                        Err(Error::TypeError(format!("String can only be multiplied by an integer")))
                                    }
                                }
                                Token::Equal => Ok(Value::Boolean(s == t)),
                                Token::NotEqual => Ok(Value::Boolean(s != t)),
                                Token::Greater => Ok(Value::Boolean(s > t)),
                                Token::Less => Ok(Value::Boolean(s < t)),
                                Token::GreaterEqual => Ok(Value::Boolean(s >= t)),
                                Token::LessEqual => Ok(Value::Boolean(s <= t)),
                                _ => Err(Error::UnsupportedOperation(format!("Unsupported operator for strings"))),
                            }
                        }

                        (Value::String(s), Value::Number(n)) => {
                            match op {
                                Token::Multiply => Ok(Value::String(s.repeat(n as usize))),
                                _ => Err(Error::UnsupportedOperation(format!("Unsupported operation between string and number"))),
                            }
                        }
                        (Value::Number(n), Value::String(s)) => {
                            match op {
                                Token::Multiply => Ok(Value::String(s.repeat(n as usize))),
                                _ => Err(Error::UnsupportedOperation(format!("Unsupported operation between number and string"))),
                            }
                        }
                        (Value::Boolean(b1), Value::Boolean(b2)) => {
                            match op {
                                Token::Equal => Ok(Value::Boolean(b1 == b2)),
                                Token::NotEqual => Ok(Value::Boolean(b1 != b2)),
                                _ => Err(Error::UnsupportedOperation(format!("Unsupported operator for booleans"))),
                            }
                        }
                        (Value::Type(t1), Value::Type(t2)) => {
                            match op {
                                Token::Equal => Ok(Value::Boolean(t1 == t2)),
                                Token::NotEqual => Ok(Value::Boolean(t1 != t2)),
                                _ => Err(Error::UnsupportedOperation(format!("Unsupported operator for types"))),
                            }
                        }
                        _ => Err(Error::UnsupportedOperation(format!("Unsupported operation for given types"))),
                    }
                }
            }
        },
        ASTNode::Array(elements) => {
            let values: Vec<Value> = elements
                .iter()
                .map(|elem| interpret_node(elem, symbol_table, is_verbose, in_loop))
                .collect::<Result<_, _>>()?;
            Ok(Value::Array(values))
        },

        ASTNode::Index(expr, index) => {
            let array = interpret_node(expr, symbol_table, is_verbose, in_loop)?;
            let index = interpret_node(index, symbol_table, is_verbose, in_loop)?;

            match (array, index) {
                (Value::Array(arr), Value::Number(i)) => {
                    if i < 0 || i >= arr.len() as i32 {
                        return Err(Error::IndexOutOfBounds(format!("Index out of bounds")));
                    }
                    Ok(arr[i as usize].clone())
                },
                (Value::String(s), Value::Number(i)) => {
                    if i < 0 || i >= s.len() as i32 {
                        return Err(Error::IndexOutOfBounds(format!("Index out of bounds")));
                    }
                    Ok(Value::String(s.chars().nth(i as usize).unwrap().to_string()))
                },
                _ => Err(Error::TypeError(format!("Invalid indexing operation"))),
            }
        },
        ASTNode::Print(expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop)?;
            if is_verbose {
                println!("call print({})", value);
            } else {
                println!("{}", value);
            }
            Ok(Value::Null)
        },
        ASTNode::UnaryOp(op, expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop)?;
            match (op, value) {
                (Token::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
                _ => Err(Error::UnsupportedOperation(format!("Unsupported unary operation"))),
            }
        },
        ASTNode::While(condition, body) => {
            loop {
                let cond_value = interpret_node(condition, symbol_table, is_verbose, true)?;
                if let Value::Boolean(false) = cond_value {
                    break;
                }

                for stmt in body {
                    let result = interpret_node(stmt, symbol_table, is_verbose, true)?;
                    match result {
                        Value::Break => return Ok(Value::Null),
                        Value::Continue => break,
                        _ => {}
                    }
                }
            }
            Ok(Value::Null)
        },
        ASTNode::Var(name, expr, is_mutable) => {
            let value = if let Some(expr) = expr {
                interpret_node(expr, symbol_table, is_verbose, in_loop)?
            } else {
                Value::Null
            };
            symbol_table.insert(name.clone(), (value.clone(), *is_mutable));
            if is_verbose {
                println!("declare variable {} with {:?}", name, value);
            }
            Ok(Value::Null)
        },
        ASTNode::Assign(name, expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop)?;
            if let Some((current_value, is_mutable)) = symbol_table.get_mut(name) {
                if !*is_mutable {
                    return Err(Error::TypeError(format!("Cannot assign to immutable variable: {}", name)));
                }
                *current_value = value.clone();
                if is_verbose {
                    println!("assign {} = {:?}", name, value);
                }
            } else {
                return Err(Error::VariableNotDeclared(format!("Variable not declared: {}", name)));
            }
            Ok(Value::Null)
        },
        ASTNode::IndexAssign(array, index, value) => {
            let array_name = if let ASTNode::Identifier(name) = &**array {
                name
            } else {
                return Err(Error::TypeError(format!("Expected array identifier in index assignment")));
            };

            let index_value = interpret_node(index, symbol_table, is_verbose, in_loop)?;
            let value = interpret_node(value, symbol_table, is_verbose, in_loop)?;

            if let Value::Number(index) = index_value {
                if let Some((Value::Array(ref mut arr), is_mutable)) = symbol_table.get_mut(array_name) {
                    if !*is_mutable {
                        return Err(Error::TypeError(format!("Cannot assign to immutable array '{}'", array_name)));
                    }
                    if index as usize >= arr.len() {
                        return Err(Error::IndexOutOfBounds(format!("Index out of bounds for array '{}'", array_name)));
                    }
                    arr[index as usize] = value;
                } else {
                    return Err(Error::TypeError(format!("Array '{}' not found or is not mutable", array_name)));
                }
            } else {
                return Err(Error::TypeError(format!("Expected integer index in array assignment")));
            }

            Ok(Value::Null)
        },
        ASTNode::Identifier(name) => {
            if let Some((value, _)) = symbol_table.get(name) {
                Ok(value.clone())
            } else {
                Err(Error::VariableNotDeclared(format!("Variable not found: {}", name)))
            }
        },
        ASTNode::TypeLiteral(type_name) => {
            Ok(Value::Type(type_name.clone()))
        },
        ASTNode::Type(expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop)?;
            let type_str = match value {
                Value::Number(_) => "int",
                Value::String(_) => "str",
                Value::Boolean(_) => "bool",
                Value::Float(_) => "float",
                Value::Null => "null",
                Value::Type(_) => "type",
                Value::Break => "break",
                Value::Continue => "continue",
                Value::Array(_) => "array",
            };
            if is_verbose {
                println!("call type({:?}) = {}", value, type_str);
            }
            Ok(Value::Type(type_str.to_string()))
        },
        ASTNode::TypeCast(type_name, expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop)?;
            match type_name.as_str() {
                "int" => match value {
                    Value::Number(n) => Ok(Value::Number(n)),
                    Value::Float(f) => Ok(Value::Number(f as i32)),
                    Value::String(s) => {
                        if s.chars().all(|c| c.is_digit(10)) {
                            Ok(Value::Number(s.parse::<i32>().unwrap()))
                        } else {
                            Err(Error::TypeError(format!("Cannot convert string '{}' to int", s)))
                        }
                    },
                    Value::Boolean(b) => Ok(Value::Number(if b { 1 } else { 0 })),
                    _ => Err(Error::TypeError(format!("Cannot convert to int"))),
                },
                "str" => match value {
                    Value::Number(n) => Ok(Value::String(n.to_string())),
                    Value::Float(f) => Ok(Value::String(f.to_string())),
                    Value::String(s) => Ok(Value::String(s)),
                    Value::Boolean(b) => Ok(Value::String(b.to_string())),
                    Value::Null => Ok(Value::String("null".to_string())),
                    _ => Err(Error::TypeError(format!("Cannot convert to string"))),
                },
                "float" => match value {
                    Value::Number(n) => Ok(Value::Float(n as f64)),
                    Value::Float(f) => Ok(Value::Float(f)),
                    Value::String(s) => {
                        match s.parse::<f64>() {
                            Ok(f) => Ok(Value::Float(f)),
                            Err(_) => Err(Error::TypeError(format!("Cannot convert string '{}' to float", s))),
                        }
                    },
                    Value::Boolean(b) => Ok(Value::Float(if b { 1.0 } else { 0.0 })),
                    _ => Err(Error::TypeError(format!("Cannot convert to float"))),
                },
                "bool" => match value {
                    Value::Number(n) => Ok(Value::Boolean(n != 0)),
                    Value::Float(f) => Ok(Value::Boolean(f != 0.0)),
                    Value::String(s) => Ok(Value::Boolean(!s.is_empty())),
                    Value::Boolean(b) => Ok(Value::Boolean(b)),
                    Value::Null => Ok(Value::Boolean(false)),
                    _ => Err(Error::TypeError(format!("Cannot convert to bool"))),
                },
                _ => Err(Error::TypeError(format!("Unknown type cast: {}", type_name))),
            }
        },
        ASTNode::If(condition, if_block, elif_blocks, else_block) => {
            let condition_value = interpret_node(condition, symbol_table, is_verbose, in_loop)?;
            if let Value::Boolean(true) = condition_value {
                for stmt in if_block {
                    let result = interpret_node(stmt, symbol_table, is_verbose, in_loop)?;
                    if matches!(result, Value::Break | Value::Continue) {
                        return Ok(result);
                    }
                }
            } else {
                let mut executed = false;
                for (elif_condition, elif_statements) in elif_blocks {
                    let elif_condition_value = interpret_node(elif_condition, symbol_table, is_verbose, in_loop)?;
                    if let Value::Boolean(true) = elif_condition_value {
                        for stmt in elif_statements {
                            let result = interpret_node(stmt, symbol_table, is_verbose, in_loop)?;
                            if matches!(result, Value::Break | Value::Continue) {
                                return Ok(result);
                            }
                        }
                        executed = true;
                        break;
                    }
                }
                if !executed {
                    if let Some(else_statements) = else_block {
                        for stmt in else_statements {
                            let result = interpret_node(stmt, symbol_table, is_verbose, in_loop)?;
                            if matches!(result, Value::Break | Value::Continue) {
                                return Ok(result);
                            }
                        }
                    }
                }
            }
            Ok(Value::Null)
        },
        ASTNode::For(init, condition, update, body) => {
            interpret_node(init, symbol_table, is_verbose, true)?;
            loop {
                let cond_value = interpret_node(condition, symbol_table, is_verbose, true)?;
                if let Value::Boolean(false) = cond_value {
                    break;
                }

                for stmt in body {
                    let result = interpret_node(stmt, symbol_table, is_verbose, true)?;
                    match result {
                        Value::Break => return Ok(Value::Null),
                        Value::Continue => break,
                        _ => {}
                    }
                }

                interpret_node(update, symbol_table, is_verbose, true)?;
            }
            Ok(Value::Null)
        },
        ASTNode::Break => {
            if !in_loop {
                return Err(Error::BreakOutsideLoop);
            }
            Ok(Value::Break)
        },
        ASTNode::Continue => {
            if !in_loop {
                return Err(Error::ContinueOutsideLoop);
            }
            Ok(Value::Continue)
        },
    }
}
