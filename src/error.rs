use std::fmt;

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
        }
    }
}

impl std::error::Error for Error {}
