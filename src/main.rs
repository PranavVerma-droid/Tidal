use std::env;
use std::fs;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if verbose mode is enabled
    // We use String::from() to create owned String instances for comparison
    let is_verbose = args.contains(&String::from("--verbose")) || args.contains(&String::from("-v"));

    // Ensure we have at least one argument (the input file)
    if args.len() < 2 {
        eprintln!("Usage: bl <file.bl> [--verbose | -v]");
        std::process::exit(1);
    }

    // Get the filename from command line arguments
    let filename = &args[1];

    // Read the contents of the file
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");

    // Create a new parser instance with the file contents
    let mut parser = parser::Parser::new(&contents);

    // Parse the contents into an Abstract Syntax Tree (AST)
    let ast = parser.parse();

    // Interpret the AST
    // The `is_verbose` flag determines whether to print detailed execution information
    interpreter::interpret(ast, is_verbose);
}