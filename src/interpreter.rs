use crate::lexer::Token;
use crate::parser::ASTNode;

pub fn interpret(ast: Vec<ASTNode>, is_verbose: bool) -> Option<i32> {
    for node in ast {
        match node {
            ASTNode::Var(name, value) => {
                let evaluated_value = evaluate(*value);
                if is_verbose {
                    println!("declare variable {} with Number({})", name, evaluated_value);
                }
            }
            ASTNode::Print(expr) => {
                let evaluated_value = evaluate(*expr);
                if is_verbose {
                    println!("call print({})", evaluated_value); // For Verbose Output
                } else {
                    println!("{}", evaluated_value); // Result Print
                }
            }
            _ => panic!("Unexpected AST node"), // not expected AST Node: Flag 1
        }
    }

    // No Result is returned : Flag 2
    None
}

fn evaluate(node: ASTNode) -> i32 {
    match node {
        ASTNode::Number(val) => val,
        ASTNode::BinaryOp(left, op, right) => {
            let left_val = evaluate(*left);
            let right_val = evaluate(*right);
            match op {
                Token::Plus => left_val + right_val,
                Token::Minus => left_val - right_val,
                Token::Multiply => left_val * right_val,
                Token::Divide => left_val / right_val,
                _ => panic!("Unsupported operator"),
            }
        }
        _ => panic!("Unexpected AST node"),
    }
}
