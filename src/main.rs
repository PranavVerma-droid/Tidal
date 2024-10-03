use std::env;
use std::fs;
use std::path::Path;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    // collect args
    let args: Vec<String> = env::args().collect();

    // verbose
    let is_verbose = args.contains(&String::from("--verbose")) || args.contains(&String::from("-v"));

    if args.len() < 2 {
        eprintln!("Usage: bl <file.bl> [--verbose | -v]");
        std::process::exit(1);
    }


    let filename = &args[1];
    if !filename.ends_with(".bl") {
        eprintln!("Error: Input file must have a .bl extension");
        std::process::exit(1);
    }

    // file check disk
    if !Path::new(filename).exists() {
        eprintln!("Error: File '{}' not found", filename);
        std::process::exit(1);
    }

    // read file
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Parser
    let mut parser = parser::Parser::new(&contents);

    // Parser to AST
    let ast = parser.parse();

    // Interpreter
    interpreter::interpret(ast, is_verbose);
}