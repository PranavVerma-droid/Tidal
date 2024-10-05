use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

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
    let is_brain_rot = filename.ends_with(".br");

    if !filename.ends_with(".td") && !is_brain_rot {
        eprintln!("Error: Input file must have a .td or .br extension");
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

    // Brain Rot Parser
    let processed_contents = if is_brain_rot {
        preprocess_brain_rot(&contents)
    } else {
        contents
    };

    // Parser
    let mut parser = parser::Parser::new(&processed_contents);

    // Parser to AST
    let ast = parser.parse();

    // Interpreter
    interpreter::interpret(ast, is_verbose);
}

fn help() {
    println!("");
    println!("Tidal Programming Language");
    println!("Made by Pranav Verma - For the Lagoon Project.");
    println!("");
    println!("Usage: td <file.td | file.br> [--verbose | -v]");
    println!("Options:");
    println!("  --verbose, -v      Enable verbose output");
    println!("  help, --help, -h   Display this help message");
    println!("");
}

fn preprocess_brain_rot(input: &str) -> String {
    let replacements: HashMap<&str, &str> = [
        ("rizzler", "var"),
        ("sigma", "novar"),
        /* ("be", "="), */
        ("no cap", ";"),
        ("skibidi", "print"),
        ("fanum tax", "type"),
        ("bussin", "for"),
        ("sussy", "/*"),
        ("baka", "*/"),
    ].iter().cloned().collect();

    let mut result = String::new();
    let mut buffer = String::new();

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut matched = false;

        for (&key, &value) in &replacements {
            if chars[i..].starts_with(&key.chars().collect::<Vec<_>>()) {
                result.push_str(value);
                i += key.len();
                matched = true;
                break;
            }
        }

        if !matched {
            if chars[i].is_alphabetic() || chars[i].is_whitespace() {
                buffer.push(chars[i]);
            } else {
                if !buffer.is_empty() {
                    let trimmed = buffer.trim();
                    if let Some(&replacement) = replacements.get(trimmed) {
                        result.push_str(replacement);
                    } else {
                        result.push_str(&buffer);
                    }
                    buffer.clear();
                }
                result.push(chars[i]);
            }
            i += 1;
        }
    }

    if !buffer.is_empty() {
        let trimmed = buffer.trim();
        if let Some(&replacement) = replacements.get(trimmed) {
            result.push_str(replacement);
        } else {
            result.push_str(&buffer);
        }
    }

    result
}