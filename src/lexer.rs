#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Var,
    Print,
    Identifier(String),
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Semicolon,
    LParen,
    RParen,
    EOF,
}

pub struct Lexer {
    input: String,
    position: usize,
    lookahead: Option<Token>, // Lookahead for peeking
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            lookahead: None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.lookahead.take() {
            return token;
        }

        self.skip_whitespace();
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let current_char = self.current_char();

        match current_char {
            '0'..='9' => return self.read_number(),
            '+' => {
                self.advance();
                return Token::Plus;
            }
            '-' => {
                self.advance();
                return Token::Minus;
            }
            '*' => {
                self.advance();
                return Token::Multiply;
            }
            '/' => {
                self.advance();
                return Token::Divide;
            }
            '=' => {
                self.advance();
                return Token::Assign;
            }
            ';' => {
                self.advance();
                return Token::Semicolon;
            }
            '(' => {
                self.advance();
                return Token::LParen;
            }
            ')' => {
                self.advance();
                return Token::RParen;
            }
            _ => {
                if self.is_alpha(current_char) {
                    let identifier = self.read_identifier();
                    return match identifier.as_str() {
                        "var" => Token::Var,
                        "print" => Token::Print,
                        _ => Token::Identifier(identifier),
                    };
                } else {
                    panic!("Unexpected character: {}", current_char);
                }
            }
        }
    }

    pub fn peek_token(&mut self) -> Token {
        if self.lookahead.is_none() {
            self.lookahead = Some(self.next_token());
        }
        self.lookahead.clone().unwrap()
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.current_char().is_digit(10) {
            self.advance();
        }
        let number: i32 = self.input[start..self.position].parse().unwrap();
        Token::Number(number)
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.is_alpha(self.current_char()) {
            self.advance();
        }
        self.input[start..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic()
    }
}
