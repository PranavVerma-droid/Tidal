use crate::parser::ASTNode;

pub struct Compiler;

impl Compiler {
    pub fn compile(ast: Vec<ASTNode>) {
        for node in ast {
            Compiler::compile_node(node);
        }
    }

    fn compile_node(node: ASTNode) {
        match node {
            ASTNode::Number(val) => println!("i32.const {}", val), // Mock IR
            ASTNode::Print(expr) => {
                println!("call print({:?})", expr);
            }
            ASTNode::BinaryOp(left, op, right) => {
                println!("binary op {:?} with {:?} and {:?}", op, left, right);
            }
            ASTNode::Var(name, expr) => {
                println!("declare variable {} with {:?}", name, expr);
            }
        }
    }
}
