use std::str::Chars;
use std::iter::Peekable;

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
    EOF,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(token) = self.handle_comment() {
            return token;
        }

        match self.input.next() {
            Some(',') => Token::Comma,
            Some('/') => {
                if self.input.peek() == Some(&'/') {
                    self.input.next(); 
                    Token::FloorDivide
                } else {
                    Token::Divide
                }
            },
            Some('*') => {
                if self.input.peek() == Some(&'*') {
                    self.input.next(); 
                    Token::Power
                } else {
                    Token::Multiply
                }
            },
            Some('&') => {
                if self.input.next_if_eq(&'&').is_some() {
                    Token::And
                } else {
                    panic!("Unexpected character: &")
                }
            },
            Some('|') => {
                if self.input.next_if_eq(&'|').is_some() {
                    Token::Or
                } else {
                    panic!("Unexpected character: |")
                }
            },
            Some(ch) => match ch {
                '0'..='9' => self.read_number(ch),
                '+' => Token::Plus,
                '-' => Token::Minus,
                '=' => {
                    if self.input.next_if_eq(&'=').is_some() {
                        Token::Equal
                    } else {
                        Token::Assign
                    }
                },
                '>' => {
                    if self.input.next_if_eq(&'=').is_some() {
                        Token::GreaterEqual
                    } else {
                        Token::Greater
                    }
                },
                '<' => {
                    if self.input.next_if_eq(&'=').is_some() {
                        Token::LessEqual
                    } else {
                        Token::Less
                    }
                },
                '!' => {
                    if self.input.next_if_eq(&'=').is_some() {
                        Token::NotEqual
                    } else {
                        panic!("Unexpected character: !")
                    }
                },
                ';' => Token::Semicolon,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                '%' => Token::Modulus,
                '"' => self.read_string(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier_or_keyword(ch),
                _ => panic!("Unexpected character: {}", ch),
            },
            None => Token::EOF,
        }
    }

    fn read_number(&mut self, first_digit: char) -> Token {
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
            Token::Float(number.parse().unwrap())
        } else {
            Token::Number(number.parse().unwrap())
        }
    }

    fn read_identifier_or_keyword(&mut self, first_char: char) -> Token {
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
            "var" => Token::Var,
            "novar" => Token::NoVar,
            "print" => Token::Print,
            "type" => Token::Type,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "null" => Token::Null,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "for" => Token::For,
            "while" => Token::While,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "int" | "str" | "float" | "bool" => {
                if self.input.peek() == Some(&'(') {
                    Token::TypeCast(identifier)
                } else {
                    Token::TypeLiteral(identifier)
                }
            },
            _ => Token::Identifier(identifier),
        }
    }

    fn handle_comment(&mut self) -> Option<Token> {
        if self.input.next_if(|&ch| ch == '/').is_some() {
            if self.input.next_if(|&ch| ch == '*').is_some() {
                self.skip_multiline_comment();
                return Some(self.next_token());
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


    fn read_string(&mut self) -> Token {
        let mut string = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch == '"' {
                self.input.next(); 
                break;
            }
            string.push(ch);
            self.input.next();
        }
        Token::String(string)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }
}