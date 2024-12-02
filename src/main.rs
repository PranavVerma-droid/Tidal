use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::process;
use std::io::{self, Write};

mod interpreter;
mod lexer;
mod parser;
mod error;
mod libs;

fn main() {
    // collect args
    let args: Vec<String> = env::args().collect();

    // verbose mode flag check
    let is_verbose = args.contains(&String::from("--verbose")) || args.contains(&String::from("-v"));

    // error display lul
    if args.len() < 2 || args.contains(&String::from("help")) || args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        help();
        process::exit(1);
    }

    let filename = &args[1];
    let is_brain_rot = filename.ends_with(".br");

    if !filename.ends_with(".td") && !is_brain_rot {
        eprintln!("Error: Input file must have a .td or .br extension");
        process::exit(1);
    }

    // file check disk
    if !Path::new(filename).exists() {
        eprintln!("Error: File '{}' not found", filename);
        process::exit(1);
    }

    // read file
    let contents = fs::read_to_string(filename)
        .map_err(|e| error::Error::FileNotFound(format!("Failed to read file: {}", e)))
        .unwrap();

    // Brain Rot Parser
    let processed_contents = if is_brain_rot {
        preprocess_skibidi(&contents)
    } else {
        contents
    };

    // Parser
    let mut parser = parser::Parser::new(&processed_contents);

    // Parser to AST
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            print_error(&e);
            process::exit(1);
        }
    };

    // Interpreter
    match interpreter::interpret(ast, is_verbose) {
        Ok(_) => {},
        Err(e) => {
            print_error(&e);
            process::exit(1);
        }
    }
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

//okay, here is where the brainrot starts ☠️☠️
fn preprocess_skibidi(input: &str) -> String {
    let replacements: HashMap<&str, &str> = [
        ("rizzler", "var"),
        ("sigma", "novar"),
        ("be", "="),
        ("no cap", ";"),
        ("skibidi", "print"),
        ("fanum tax", "type"),
        ("bussin", "for"),
        ("yeet", "return"),
        ("sussy", "/*"),
        ("baka", "*/"),
        ("aura +69420", "break"),
        ("aura -69420", "continue"),
        ("drip", "if"),
        ("mid", "elif"),
        ("nah", "else"),
        ("gyatt", "true"),
        ("diddy", "false"),
        ("big yikes", "func"),
        ("spill", "while"),
        ("goat", "input"),
        ("boogey", "import"),
    ].iter().cloned().collect();

    let mut result = String::new();
    let mut in_word = false;
    let mut word_buffer = String::new();

    for c in input.chars() {
        if c.is_whitespace() {
            if in_word {
                if let Some(&replacement) = replacements.get(word_buffer.trim()) {
                    result.push_str(replacement);
                } else {
                    result.push_str(&word_buffer);
                }
                word_buffer.clear();
                in_word = false;
            }
            result.push(' ');
        } else {
            if "=(),[]{}".contains(c) {
                if in_word {
                    if let Some(&replacement) = replacements.get(word_buffer.trim()) {
                        result.push_str(replacement);
                    } else {
                        result.push_str(&word_buffer);
                    }
                    word_buffer.clear();
                    in_word = false;
                }
                result.push(c);
                continue;
            }

            word_buffer.push(c);
            in_word = true;
        }
    }

    if in_word {
        if let Some(&replacement) = replacements.get(word_buffer.trim()) {
            result.push_str(replacement);
        } else {
            result.push_str(&word_buffer);
        }
    }

    for (&pattern, &replacement) in &replacements {
        if pattern.contains(' ') {
            result = result.replace(pattern, replacement);
        }
    }

    result
}

fn print_error(error: &error::Error) {
    let stderr = io::stderr();
    let mut handle = stderr.lock();

    writeln!(handle, "\x1b[31m{}\x1b[0m", error).unwrap();
}
