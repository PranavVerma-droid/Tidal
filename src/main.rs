use std::env;
use std::fs;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let is_verbose = args.contains(&"--verbose".to_string()) || args.contains(&"-v".to_string());

    if args.len() < 2 {
        eprintln!("Usage: blue_lagoon <file.bl> [--verbose | -v]");
        std::process::exit(1);
    }

    // Process Input Filename
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    // Parser
    let mut parser = parser::Parser::new(&contents);
    let ast = parser.parse();

    // Interpreter
    interpreter::interpret(ast, is_verbose);
}