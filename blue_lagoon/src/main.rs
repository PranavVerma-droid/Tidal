mod lexer;
mod parser;
mod compiler;

use lexer::Lexer;
use parser::Parser;
use compiler::Compiler;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ./blue_lagoon <file.bl>");
        return;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read file");

    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    
    Compiler::compile(ast);
}
