use crate::parser::{ASTNode, Value};
use crate::lexer::Token;
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

pub fn interpret(ast: Vec<ASTNode>, is_verbose: bool) -> Option<Value> {
    let mut symbol_table: HashMap<String, (Value, bool)> = HashMap::new(); // (Value, is_mutable)
    let mut result = None;

    for node in ast {
        result = Some(interpret_node(&node, &mut symbol_table, is_verbose, false));
        if let Value::Break = result.as_ref().unwrap() {
            panic!("Break statement outside of loop");
        }
        if let Value::Continue = result.as_ref().unwrap() {
            panic!("Continue statement outside of loop");
        }
    }

    result
}

fn interpret_node(node: &ASTNode, symbol_table: &mut HashMap<String, (Value, bool)>, is_verbose: bool, in_loop: bool) -> Value {
    match node {
        ASTNode::Number(val) => Value::Number(*val),
        ASTNode::String(val) => Value::String(val.clone()),
        ASTNode::Float(val) => Value::Float(*val),
        ASTNode::Boolean(val) => Value::Boolean(*val),
        ASTNode::Null => Value::Null,
        ASTNode::BinaryOp(left, op, right) => {
            let left_val = interpret_node(left, symbol_table, is_verbose, in_loop);
            match op {
                Token::And => {
                    if let Value::Boolean(false) = left_val {
                        return Value::Boolean(false);
                    }
                    let right_val = interpret_node(right, symbol_table, is_verbose, in_loop);
                    match (left_val, right_val) {
                        (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l && r),
                        _ => panic!("AND operator can only be applied to boolean values"),
                    }
                },
                Token::Or => {
                    if let Value::Boolean(true) = left_val {
                        return Value::Boolean(true);
                    }
                    let right_val = interpret_node(right, symbol_table, is_verbose, in_loop);
                    match (left_val, right_val) {
                        (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l || r),
                        _ => panic!("OR operator can only be applied to boolean values"),
                    }
                },
                _ => {
                    let right_val = interpret_node(right, symbol_table, is_verbose, in_loop);
                    match (left_val, right_val) {
                        (Value::Number(l), Value::Number(r)) => {
                            match op {
                                Token::Plus => Value::Number(l + r),
                                Token::Minus => Value::Number(l - r),
                                Token::Multiply => Value::Number(l * r),
                                Token::Divide => Value::Float(l as f64 / r as f64),
                                Token::Equal => Value::Boolean(l == r),
                                Token::NotEqual => Value::Boolean(l != r),
                                Token::Greater => Value::Boolean(l > r),
                                Token::Less => Value::Boolean(l < r),
                                Token::GreaterEqual => Value::Boolean(l >= r),
                                Token::FloorDivide => Value::Number(l / r),
                                Token::LessEqual => Value::Boolean(l <= r),
                                Token::Modulus => Value::Number(l % r),
                                Token::Power => Value::Number(l.pow(r as u32)),
                                _ => panic!("Unsupported operator for numbers"),
                            }
                        }
                        (Value::Float(l), Value::Float(r)) => { 
                            match op {
                                Token::Plus => Value::Float(l + r),
                                Token::Minus => Value::Float(l - r),
                                Token::Multiply => Value::Float(l * r),
                                Token::Divide => Value::Float(l / r),
                                Token::Equal => Value::Boolean(l == r),
                                Token::NotEqual => Value::Boolean(l != r),
                                Token::Greater => Value::Boolean(l > r),
                                Token::Modulus => Value::Float(l % r),
                                Token::FloorDivide => Value::Number((l / r).floor() as i32),
                                Token::Less => Value::Boolean(l < r),
                                Token::GreaterEqual => Value::Boolean(l >= r),
                                Token::LessEqual => Value::Boolean(l <= r),
                                Token::Power => Value::Float(l.powf(r)),
                                _ => panic!("Unsupported operator for floats"),
                            }
                        }
                        (Value::Number(l), Value::Float(r)) => { 
                            let l = l as f64;
                            match op {
                                Token::Plus => Value::Float(l + r),
                                Token::Minus => Value::Float(l - r),
                                Token::Multiply => Value::Float(l * r),
                                Token::Divide => Value::Float(l / r),
                                Token::Equal => Value::Boolean(l == r),
                                Token::Modulus => Value::Float(l % r),
                                Token::NotEqual => Value::Boolean(l != r),
                                Token::FloorDivide => Value::Number((l / r).floor() as i32),
                                Token::Greater => Value::Boolean(l > r),
                                Token::Less => Value::Boolean(l < r),
                                Token::GreaterEqual => Value::Boolean(l >= r),
                                Token::Power => Value::Float(l.powf(r)),
                                Token::LessEqual => Value::Boolean(l <= r),
                                _ => panic!("Unsupported operator for mixed number and float"),
                            }
                        }
                        (Value::Float(l), Value::Number(r)) => { 
                            let r = r as f64;
                            match op {
                                Token::Plus => Value::Float(l + r),
                                Token::Minus => Value::Float(l - r),
                                Token::Multiply => Value::Float(l * r),
                                Token::Divide => Value::Float(l / r),
                                Token::Equal => Value::Boolean(l == r),
                                Token::Modulus => Value::Float(l % r),
                                Token::NotEqual => Value::Boolean(l != r),
                                Token::Greater => Value::Boolean(l > r),
                                Token::Less => Value::Boolean(l < r),
                                Token::GreaterEqual => Value::Boolean(l >= r),
                                Token::FloorDivide => Value::Number((l / r).floor() as i32),
                                Token::Power => Value::Float(l.powf(r)),
                                Token::LessEqual => Value::Boolean(l <= r),
                                _ => panic!("Unsupported operator for mixed float and number"),
                            }
                        }
                        (Value::String(s), Value::String(t)) => {
                            match op {
                                Token::Plus => Value::String(s + &t),
                                Token::Multiply => {
                                    if let Value::Number(n) = interpret_node(right, symbol_table, is_verbose, in_loop) {
                                        Value::String(s.repeat(n as usize))
                                    } else {
                                        panic!("String can only be multiplied by an integer")
                                    }
                                }
                                Token::Equal => Value::Boolean(s == t),
                                Token::NotEqual => Value::Boolean(s != t),
                                _ => panic!("Unsupported operator for strings"),
                            }
                        }
                        (Value::String(s), Value::Number(n)) => {
                            match op {
                                Token::Multiply => Value::String(s.repeat(n as usize)),
                                _ => panic!("Unsupported operation between string and number"),
                            }
                        }
                        (Value::Number(n), Value::String(s)) => {
                            match op {
                                Token::Multiply => Value::String(s.repeat(n as usize)),
                                _ => panic!("Unsupported operation between number and string"),
                            }
                        }
                        (Value::Boolean(b1), Value::Boolean(b2)) => {
                            match op {
                                Token::Equal => Value::Boolean(b1 == b2),
                                Token::NotEqual => Value::Boolean(b1 != b2),
                                _ => panic!("Unsupported operator for booleans"),
                            }
                        }
                        (Value::Type(t1), Value::Type(t2)) => {
                            match op {
                                Token::Equal => Value::Boolean(t1 == t2),
                                Token::NotEqual => Value::Boolean(t1 != t2),
                                _ => panic!("Unsupported operator for types"),
                            }
                        }
                        _ => panic!("Unsupported operation for given types"),
                    }
                }
            }
        },
        ASTNode::Array(elements) => {
            let values: Vec<Value> = elements
                .iter()
                .map(|elem| interpret_node(elem, symbol_table, is_verbose, in_loop))
                .collect();
            Value::Array(values)
        },

        ASTNode::Index(expr, index) => {
            let array = interpret_node(expr, symbol_table, is_verbose, in_loop);
            let index = interpret_node(index, symbol_table, is_verbose, in_loop);
            
            match (array, index) {
                (Value::Array(arr), Value::Number(i)) => {
                    if i < 0 || i >= arr.len() as i32 {
                        panic!("Index out of bounds");
                    }
                    arr[i as usize].clone()
                },
                (Value::String(s), Value::Number(i)) => {
                    if i < 0 || i >= s.len() as i32 {
                        panic!("Index out of bounds");
                    }
                    Value::String(s.chars().nth(i as usize).unwrap().to_string())
                },
                _ => panic!("Invalid indexing operation"),
            }
        },
        ASTNode::Print(expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop);
            if is_verbose {
                println!("call print({})", value);
            } else {
                println!("{}", value);
            }
            Value::Null
        },
        ASTNode::UnaryOp(op, expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop);
            match (op, value) {
                (Token::Not, Value::Boolean(b)) => Value::Boolean(!b),
                _ => panic!("Unsupported unary operation"),
            }
        },
        ASTNode::While(condition, body) => {
            loop {
                let cond_value = interpret_node(condition, symbol_table, is_verbose, true);
                if let Value::Boolean(false) = cond_value {
                    break;
                }
                
                for stmt in body {
                    let result = interpret_node(stmt, symbol_table, is_verbose, true);
                    match result {
                        Value::Break => return Value::Null,
                        Value::Continue => break,
                        _ => {}
                    }
                }
            }
            Value::Null
        },
        ASTNode::Var(name, expr, is_mutable) => {
            let value = if let Some(expr) = expr {
                interpret_node(expr, symbol_table, is_verbose, in_loop)
            } else {
                Value::Null
            };
            symbol_table.insert(name.clone(), (value.clone(), *is_mutable));
            if is_verbose {
                println!("declare variable {} with {:?}", name, value);
            }
            Value::Null
        },
        ASTNode::Assign(name, expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop);
            if let Some((current_value, is_mutable)) = symbol_table.get_mut(name) {
                if !*is_mutable {
                    panic!("Cannot assign to immutable variable: {}", name);
                }
                *current_value = value.clone();
                if is_verbose {
                    println!("assign {} = {:?}", name, value);
                }
            } else {
                panic!("Variable not declared: {}", name);
            }
            Value::Null
        },
        ASTNode::IndexAssign(array, index, value) => {
            let array_name = if let ASTNode::Identifier(name) = &**array {
                name
            } else {
                panic!("Expected array identifier in index assignment");
            };

            let index_value = interpret_node(index, symbol_table, is_verbose, in_loop);
            let value = interpret_node(value, symbol_table, is_verbose, in_loop);

            if let Value::Number(index) = index_value {
                if let Some((Value::Array(ref mut arr), is_mutable)) = symbol_table.get_mut(array_name) {
                    if !*is_mutable {
                        panic!("Cannot assign to immutable array '{}'", array_name);
                    }
                    if index as usize >= arr.len() {
                        panic!("Index out of bounds for array '{}'", array_name);
                    }
                    arr[index as usize] = value;
                } else {
                    panic!("Array '{}' not found or is not mutable", array_name);
                }
            } else {
                panic!("Expected integer index in array assignment");
            }

            Value::Null
        },
        ASTNode::Identifier(name) => {
            if let Some((value, _)) = symbol_table.get(name) {
                value.clone()
            } else {
                panic!("Variable not found: {}", name);
            }
        },
        ASTNode::TypeLiteral(type_name) => {
            Value::Type(type_name.clone())
        },
        ASTNode::Type(expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop);
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
            Value::Type(type_str.to_string())
        },
        ASTNode::TypeCast(type_name, expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose, in_loop);
            match type_name.as_str() {
                "int" => match value {
                    Value::Number(n) => Value::Number(n),
                    Value::Float(f) => Value::Number(f as i32),
                    Value::String(s) => {
                        if s.chars().all(|c| c.is_digit(10)) {
                            Value::Number(s.parse::<i32>().unwrap())
                        } else {
                            panic!("Cannot convert string '{}' to int", s)
                        }
                    },
                    Value::Boolean(b) => Value::Number(if b { 1 } else { 0 }),
                    _ => panic!("Cannot convert to int"),
                },
                "str" => match value {
                    Value::Number(n) => Value::String(n.to_string()),
                    Value::Float(f) => Value::String(f.to_string()),
                    Value::String(s) => Value::String(s),
                    Value::Boolean(b) => Value::String(b.to_string()),
                    Value::Null => Value::String("null".to_string()),
                    _ => panic!("Cannot convert to string"),
                },
                "float" => match value {
                    Value::Number(n) => Value::Float(n as f64),
                    Value::Float(f) => Value::Float(f),
                    Value::String(s) => {
                        match s.parse::<f64>() {
                            Ok(f) => Value::Float(f),
                            Err(_) => panic!("Cannot convert string '{}' to float", s),
                        }
                    },
                    Value::Boolean(b) => Value::Float(if b { 1.0 } else { 0.0 }),
                    _ => panic!("Cannot convert to float"),
                },
                "bool" => match value {
                    Value::Number(n) => Value::Boolean(n != 0),
                    Value::Float(f) => Value::Boolean(f != 0.0),
                    Value::String(s) => Value::Boolean(!s.is_empty()),
                    Value::Boolean(b) => Value::Boolean(b),
                    Value::Null => Value::Boolean(false),
                    _ => panic!("Cannot convert to bool"),
                },
                _ => panic!("Unknown type cast: {}", type_name),
            }
        },
        ASTNode::If(condition, if_block, elif_blocks, else_block) => {
            let condition_value = interpret_node(condition, symbol_table, is_verbose, in_loop);
            if let Value::Boolean(true) = condition_value {
                for stmt in if_block {
                    let result = interpret_node(stmt, symbol_table, is_verbose, in_loop);
                    if matches!(result, Value::Break | Value::Continue) {
                        return result;
                    }
                }
            } else {
                let mut executed = false;
                for (elif_condition, elif_statements) in elif_blocks {
                    let elif_condition_value = interpret_node(elif_condition, symbol_table, is_verbose, in_loop);
                    if let Value::Boolean(true) = elif_condition_value {
                        for stmt in elif_statements {
                            let result = interpret_node(stmt, symbol_table, is_verbose, in_loop);
                            if matches!(result, Value::Break | Value::Continue) {
                                return result;
                            }
                        }
                        executed = true;
                        break;
                    }
                }
                if !executed {
                    if let Some(else_statements) = else_block {
                        for stmt in else_statements {
                            let result = interpret_node(stmt, symbol_table, is_verbose, in_loop);
                            if matches!(result, Value::Break | Value::Continue) {
                                return result;
                            }
                        }
                    }
                }
            }
            Value::Null
        },
        ASTNode::For(init, condition, update, body) => {
            interpret_node(init, symbol_table, is_verbose, true);
            loop {
                let cond_value = interpret_node(condition, symbol_table, is_verbose, true);
                if let Value::Boolean(false) = cond_value {
                    break;
                }
                
                for stmt in body {
                    let result = interpret_node(stmt, symbol_table, is_verbose, true);
                    match result {
                        Value::Break => return Value::Null,
                        Value::Continue => break,
                        _ => {}
                    }
                }
                
                interpret_node(update, symbol_table, is_verbose, true);
            }
            Value::Null
        },
        ASTNode::Break => {
            if !in_loop {
                panic!("Break statement outside of loop");
            }
            Value::Break
        },
        ASTNode::Continue => {
            if !in_loop {
                panic!("Continue statement outside of loop");
            }
            Value::Continue
        },
    }
}