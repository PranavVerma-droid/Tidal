use crate::parser::{ASTNode, Value};
use std::collections::HashMap;

pub fn interpret(ast: Vec<ASTNode>, is_verbose: bool) -> Option<Value> {
    let mut symbol_table: HashMap<String, (Value, bool)> = HashMap::new(); // (Value, is_mutable)
    //hash_map_gen
    let mut result = None;

    for node in ast {
        result = Some(interpret_node(&node, &mut symbol_table, is_verbose));
    }

    result
}

fn interpret_node(node: &ASTNode, symbol_table: &mut HashMap<String, (Value, bool)>, is_verbose: bool) -> Value {
    match node {
        ASTNode::Number(val) => Value::Number(*val),
        ASTNode::Null => Value::Null,
        ASTNode::BinaryOp(left, op, right) => {
            let left_val = interpret_node(left, symbol_table, is_verbose);
            let right_val = interpret_node(right, symbol_table, is_verbose);
            match (left_val, right_val) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Number(match op {
                        crate::lexer::Token::Plus => l + r,
                        crate::lexer::Token::Minus => l - r,
                        crate::lexer::Token::Multiply => l * r,
                        crate::lexer::Token::Divide => l / r,
                        _ => panic!("Unsupported operator"),
                    })
                }
                _ => Value::Null,
            }
        }
        ASTNode::Print(expr) => {
            let value = interpret_node(expr, symbol_table, is_verbose);
            if is_verbose {
                println!("call print({:?})", value);
            } else {
                match value {
                    Value::Number(n) => println!("{}", n),
                    Value::Null => println!("null"),
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
    }
}