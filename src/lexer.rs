use std::str::Chars;
use std::iter::Peekable;
use crate::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Var,
    NoVar,
    Print,
    Type,
    If,
    Elif,
    Else,
    Identifier(String),
    Number(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    TypeLiteral(String),
    TypeCast(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    FloorDivide,
    Modulus,
    Assign,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Null,
    For,
    While,
    Break,
    Continue,
    Comma,
    Power,
    And,
    Or,
    Not,
    Func,
    Return,
    Input,
    Len,
    Del,
    EOF,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    pub line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespace();

        if let Some(token) = self.handle_comment() {
            return Ok(token);
        }

        match self.input.next() {
            Some(',') => Ok(Token::Comma),
            Some('/') => {
                if self.input.peek() == Some(&'/') {
                    self.input.next();
                    Ok(Token::FloorDivide)
                } else {
                    Ok(Token::Divide)
                }
            },
            Some('*') => {
                if self.input.peek() == Some(&'*') {
                    self.input.next();
                    Ok(Token::Power)
                } else {
                    Ok(Token::Multiply)
                }
            },
            Some('&') => {
                if self.input.next_if_eq(&'&').is_some() {
                    Ok(Token::And)
                } else {
                    Err(Error::LexerError(format!("Unexpected character: & at line {}, column {}", self.line, self.column)))
                }
            },
            Some('|') => {
                if self.input.next_if_eq(&'|').is_some() {
                    Ok(Token::Or)
                } else {
                    Err(Error::LexerError(format!("Unexpected character: | at line {}, column {}", self.line, self.column)))
                }
            },
            Some('!') => {
                if self.input.next_if_eq(&'=').is_some() {
                    Ok(Token::NotEqual)
                } else {
                    Ok(Token::Not)
                }
            },
            Some(ch) => match ch {
                '0'..='9' => self.read_number(ch),
                '+' => Ok(Token::Plus),
                '-' => Ok(Token::Minus),
                '=' => {
                    if self.input.next_if_eq(&'=').is_some() {
                        Ok(Token::Equal)
                    } else {
                        Ok(Token::Assign)
                    }
                },
                '>' => {
                    if self.input.next_if_eq(&'=').is_some() {
                        Ok(Token::GreaterEqual)
                    } else {
                        Ok(Token::Greater)
                    }
                },
                '<' => {
                    if self.input.next_if_eq(&'=').is_some() {
                        Ok(Token::LessEqual)
                    } else {
                        Ok(Token::Less)
                    }
                },
                ';' => Ok(Token::Semicolon),
                '(' => Ok(Token::LParen),
                ')' => Ok(Token::RParen),
                '{' => Ok(Token::LBrace),
                '}' => Ok(Token::RBrace),
                '[' => Ok(Token::LBracket),
                ']' => Ok(Token::RBracket),
                '%' => Ok(Token::Modulus),
                '"' => self.read_string(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier_or_keyword(ch),
                _ => Err(Error::LexerError(format!("Unexpected character: {} at line {}, column {}", ch, self.line, self.column))),
            },
            None => Ok(Token::EOF),
        }
    }

    fn read_number(&mut self, first_digit: char) -> Result<Token, Error> {
        let mut number = first_digit.to_string();
        let mut is_float = false;
        while let Some(&ch) = self.input.peek() {
            if ch.is_digit(10) {
                number.push(ch);
                self.input.next();
            } else if ch == '.' && !is_float {
                is_float = true;
                number.push(ch);
                self.input.next();
            } else {
                break;
            }
        }
        if is_float {
            Ok(Token::Float(number.parse().unwrap()))
        } else {
            Ok(Token::Number(number.parse().unwrap()))
        }
    }

    fn read_identifier_or_keyword(&mut self, first_char: char) -> Result<Token, Error> {
        let mut identifier = first_char.to_string();
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.input.next();
            } else {
                break;
            }
        }
        match identifier.as_str() {
            "var" => Ok(Token::Var),
            "novar" => Ok(Token::NoVar),
            "print" => Ok(Token::Print),
            "type" => Ok(Token::Type),
            "if" => Ok(Token::If),
            "elif" => Ok(Token::Elif),
            "else" => Ok(Token::Else),
            "null" => Ok(Token::Null),
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            "for" => Ok(Token::For),
            "while" => Ok(Token::While),
            "break" => Ok(Token::Break),
            "continue" => Ok(Token::Continue),
            "int" | "str" | "float" | "bool" => {
                if self.input.peek() == Some(&'(') {
                    Ok(Token::TypeCast(identifier))
                } else {
                    Ok(Token::TypeLiteral(identifier))
                }
            },
            "func" => Ok(Token::Func),
            "return" => Ok(Token::Return),
            "input" => Ok(Token::Input),
            "len" => Ok(Token::Len),
            "del" => Ok(Token::Del),
            _ => Ok(Token::Identifier(identifier)),
        }
    }

    fn handle_comment(&mut self) -> Option<Token> {
        if self.input.next_if(|&ch| ch == '/').is_some() {
            if self.input.next_if(|&ch| ch == '*').is_some() {
                self.skip_multiline_comment();
                return Some(self.next_token().unwrap());
            } else {
                return Some(Token::Divide);
            }
        }
        None
    }

    fn skip_multiline_comment(&mut self) {
        let mut depth = 1;
        while depth > 0 {
            match (self.input.next(), self.input.peek()) {
                (Some('*'), Some(&'/')) => {
                    self.input.next();
                    depth -= 1;
                },
                (Some('/'), Some(&'*')) => {
                    self.input.next();
                    depth += 1;
                },
                (Some(_), _) => {},
                (None, _) => panic!("Unterminated comment"),
            }
        }
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        let mut string = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch == '"' {
                self.input.next();
                break;
            }
            string.push(ch);
            self.input.next();
        }
        Ok(Token::String(string))
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.input.next();
            } else {
                break;
            }
        }
    }
}
