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

    if args.len() < 2 || args.contains(&String::from("help")) || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        help();
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

    fn help() {
        println!("");
        println!("Blue Lagoon Programming Language");
        println!("Made by Pranav Verma - For the Lagoon Project.");
        println!("");
        println!("Usage: bl <file.bl> [--verbose | -v]");
        println!("Options:");
        println!("  --verbose, -v      Enable verbose output");
        println!("  help, --help, -h   Display this help message");
        println!("");
    }
}