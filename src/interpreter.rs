use crate::parser::{ASTNode, Value};
use crate::lexer::Token;
use std::collections::HashMap;

pub fn interpret(ast: Vec<ASTNode>, is_verbose: bool) -> Option<Value> {
    let mut symbol_table: HashMap<String, (Value, bool)> = HashMap::new(); // (Value, is_mutable)
    let mut result = None;

    for node in ast {
        result = Some(interpret_node(&node, &mut symbol_table, is_verbose));
    }

    result
}

fn interpret_node(node: &ASTNode, symbol_table: &mut HashMap<String, (Value, bool)>, is_verbose: bool) -> Value {
    match node {
        ASTNode::Number(val) => Value::Number(*val),
        ASTNode::String(val) => Value::String(val.clone()),
        ASTNode::Boolean(val) => Value::Boolean(*val),
        ASTNode::Null => Value::Null,
        ASTNode::BinaryOp(left, op, right) => {
            let left_val = interpret_node(left, symbol_table, is_verbose);
            let right_val = interpret_node(right, symbol_table, is_verbose);
            match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => {
                    match op {
                        Token::Plus => Value::Number(l + r),
                        Token::Minus => Value::Number(l - r),
                        Token::Multiply => Value::Number(l * r),
                        Token::Divide => Value::Number(l / r),
                        Token::Equal => Value::Boolean(l == r),
                        Token::NotEqual => Value::Boolean(l != r),
                        Token::Greater => Value::Boolean(l > r),
                        Token::Less => Value::Boolean(l < r),
                        Token::GreaterEqual => Value::Boolean(l >= r),
                        Token::LessEqual => Value::Boolean(l <= r),
                        _ => panic!("Unsupported operator for numbers"),
                    }
                }
                (Value::String(s), Value::String(t)) => {
                    match op {
                        Token::Plus => Value::String(s + &t),
                        Token::Equal => Value::Boolean(s == t),
                        Token::NotEqual => Value::Boolean(s != t),
                        _ => panic!("Unsupported operator for strings"),
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
        ASTNode::Print(expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose);
            if is_verbose {
                println!("call print({:?})", value);
            } else {
                match value {
                    Value::Number(n) => println!("{}", n),
                    Value::String(s) => println!("{}", s),
                    Value::Boolean(b) => println!("{}", b),
                    Value::Null => println!("null"),
                    Value::Type(t) => println!("{}", t),
                }
            }
            Value::Null // null after print
        }
        ASTNode::Var(name, expr, is_mutable) => {
            let value = if let Some(expr) = expr {
                interpret_node(expr, symbol_table, is_verbose)
            } else {
                Value::Null
            };
            symbol_table.insert(name.clone(), (value.clone(), *is_mutable));
            if is_verbose {
                println!("declare variable {} with {:?}", name, value);
            }
            Value::Null // null after exec
        }
        ASTNode::Assign(name, expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose);
            if let Some((_, is_mutable)) = symbol_table.get(name) {
                if !is_mutable {
                    panic!("Cannot assign to immutable variable: {}", name);
                }
                symbol_table.insert(name.clone(), (value.clone(), *is_mutable));
                if is_verbose {
                    println!("assign {} = {:?}", name, value);
                }
            } else {
                panic!("Variable not declared: {}", name);
            }
            Value::Null // null after assign
        }
        ASTNode::Identifier(name) => {
            if let Some((value, _)) = symbol_table.get(name) {
                value.clone()
            } else {
                panic!("Variable not found: {}", name);
            }
        }
        ASTNode::Index(expr, index) => {
            let value = interpret_node(expr, symbol_table, is_verbose);
            let index = interpret_node(index, symbol_table, is_verbose);
            match (value, index) {
                (Value::String(s), Value::Number(i)) => {
                    if i < 0 || i >= s.len() as i32 {
                        panic!("Index out of bounds");
                    }
                    Value::String(s.chars().nth(i as usize).unwrap().to_string())
                }
                _ => panic!("Invalid indexing operation"),
            }
        }
        ASTNode::Type(expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose);
            let type_str = match value {
                Value::Number(_) => "int",
                Value::String(_) => "str",
                Value::Boolean(_) => "bool",
                Value::Null => "null",
                Value::Type(_) => "type",
            };
            if is_verbose {
                println!("call type({:?}) = {}", value, type_str);
            }
            Value::Type(type_str.to_string())
        }
        ASTNode::TypeLiteral(type_name) => Value::Type(type_name.clone()),
        ASTNode::If(condition, if_block, elif_blocks, else_block) => {
            let condition_value = interpret_node(condition, symbol_table, is_verbose);
            if let Value::Boolean(true) = condition_value {
                for stmt in if_block {
                    interpret_node(stmt, symbol_table, is_verbose);
                }
            } else {
                let mut executed = false;
                for (elif_condition, elif_statements) in elif_blocks {
                    let elif_condition_value = interpret_node(elif_condition, symbol_table, is_verbose);
                    if let Value::Boolean(true) = elif_condition_value {
                        for stmt in elif_statements {
                            interpret_node(stmt, symbol_table, is_verbose);
                        }
                        executed = true;
                        break;
                    }
                }
                if !executed {
                    if let Some(else_statements) = else_block {
                        for stmt in else_statements {
                            interpret_node(stmt, symbol_table, is_verbose);
                        }
                    }
                }
            }
            Value::Null
        }
    }
}