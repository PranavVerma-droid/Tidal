use std::fmt;
use crate::parser::Value;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    SyntaxError(String),
    IndexOutOfBounds(String),
    VariableNotDeclared(String),
    VariableAlreadyDeclared(String),
    TypeError(String),
    UnsupportedOperation(String),
    BreakOutsideLoop,
    ContinueOutsideLoop,
    FileNotFound(String),
    InvalidFileExtension(String),
    LexerError(String),
    ParserError(String),
    InterpreterError(String),
    UnknownError(String),
    CannotGetLength(String, Value),
    DelRequiresVariableName,
    FunctionCallError(String),
    InvalidArrayIdentifier,
    InvalidFunctionArguments(String, usize, usize),
    InvalidIndex,
    LibraryError(String),
    ReturnOutsideFunction,
    UnexpectedValue(String),
    UnsupportedUnaryOperation,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SyntaxError(msg) => write!(f, "SyntaxError: {}", msg),
            Error::IndexOutOfBounds(msg) => write!(f, "IndexOutOfBounds: {}", msg),
            Error::VariableNotDeclared(msg) => write!(f, "VariableNotDeclared: {}", msg),
            Error::VariableAlreadyDeclared(msg) => write!(f, "VariableAlreadyDeclared: {}", msg),
            Error::TypeError(msg) => write!(f, "TypeError: {}", msg),
            Error::UnsupportedOperation(msg) => write!(f, "UnsupportedOperation: {}", msg),
            Error::BreakOutsideLoop => write!(f, "Break statement outside of loop"),
            Error::ContinueOutsideLoop => write!(f, "Continue statement outside of loop"),
            Error::FileNotFound(msg) => write!(f, "FileNotFound: {}", msg),
            Error::InvalidFileExtension(msg) => write!(f, "InvalidFileExtension: {}", msg),
            Error::LexerError(msg) => write!(f, "LexerError: {}", msg),
            Error::ParserError(msg) => write!(f, "ParserError: {}", msg),
            Error::InterpreterError(msg) => write!(f, "InterpreterError: {}", msg),
            Error::UnknownError(msg) => write!(f, "UnknownError: {}", msg),
            Error::CannotGetLength(type_name, value) => write!(f, "Cannot get length of {} value: {}", type_name, value),
            Error::DelRequiresVariableName => write!(f, "del() requires a variable name"),
            Error::FunctionCallError(msg) => write!(f, "Function call error: {}", msg),
            Error::InvalidArrayIdentifier => write!(f, "Expected array identifier in index assignment"),
            Error::InvalidFunctionArguments(name, expected, got) => 
                write!(f, "Function '{}' expects {} arguments but got {}", name, expected, got),
            Error::InvalidIndex => write!(f, "Expected integer index in array assignment"),
            Error::LibraryError(msg) => write!(f, "Library error: {}", msg),
            Error::ReturnOutsideFunction => write!(f, "'return' outside function"),
            Error::UnexpectedValue(msg) => write!(f, "Unexpected value: {}", msg),
            Error::UnsupportedUnaryOperation => write!(f, "Unsupported unary operation"),
        }
    }
}

impl std::error::Error for Error {}
