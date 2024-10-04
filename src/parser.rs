use crate::lexer::{Lexer, Token};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    String(String),
    Boolean(bool),
    Null,
    Type(String), 
}

#[derive(Debug)]
pub enum ASTNode {
    Number(i32),
    String(String),
    Boolean(bool),
    Null,
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
    Print(Box<ASTNode>),
    Var(String, Option<Box<ASTNode>>, bool),
    Assign(String, Box<ASTNode>),
    Identifier(String),
    Index(Box<ASTNode>, Box<ASTNode>),
    Type(Box<ASTNode>), 
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    symbol_table: HashMap<String, bool>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            symbol_table: HashMap::new(),
        }
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == token {
            self.current_token = self.lexer.next_token();
        } else {
            panic!("Unexpected token: {:?}, expected: {:?}", self.current_token, token);
        }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut ast_nodes = Vec::new();
        while self.current_token != Token::EOF {
            ast_nodes.push(self.parse_statement());
        }
        ast_nodes
    }

    fn parse_statement(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Var | Token::NoVar => self.parse_var_decl(),
            Token::Identifier(_) => self.parse_assign_stmt(),
            //please save me please oh god
            Token::Print => self.parse_print(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_assign_stmt(&mut self) -> ASTNode {
        if let Token::Identifier(var_name) = &self.current_token {
            let name = var_name.clone();
            self.eat(Token::Identifier(var_name.clone()));
            self.eat(Token::Assign);
            let expr = self.parse_expr();
            
            if !self.symbol_table.contains_key(&name) {
                panic!("Variable not declared: {}", name);
            }
            
            self.eat(Token::Semicolon);
            ASTNode::Assign(name, Box::new(expr))
        } else {
            panic!("Expected variable name");
            // TODO lexer to parser mut here
        }
    }

    fn parse_var_decl(&mut self) -> ASTNode {
        let is_mutable = self.current_token == Token::Var;
        //TODO Add mutable count
        self.eat(if is_mutable { Token::Var } else { Token::NoVar });
        
        if let Token::Identifier(var_name) = &self.current_token {
            let name = var_name.clone();
            self.eat(Token::Identifier(var_name.clone()));
            
            if self.symbol_table.contains_key(&name) {
                //already declared
                panic!("Variable already declared: {}", name);
            }
            
            let expr = if self.current_token == Token::Assign {
                self.eat(Token::Assign);
                Some(Box::new(self.parse_expr()))
            } else {
                None
            };
            
            self.eat(Token::Semicolon);
            self.symbol_table.insert(name.clone(), is_mutable);
            ASTNode::Var(name, expr, is_mutable)
        } else {
            //at this point, the compiler is compiling the compiler which will be
            //compiling itself

            //please dont listen to anything I say.
            panic!("Expected variable name");
        }
    }

    fn parse_print(&mut self) -> ASTNode {
        self.eat(Token::Print);
        self.eat(Token::LParen);
        let expr = self.parse_expr();
        self.eat(Token::RParen);
        self.eat(Token::Semicolon);
        ASTNode::Print(Box::new(expr))
    }

    fn parse_expr(&mut self) -> ASTNode {
        let left = self.parse_term();
        if self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = self.current_token.clone();
            self.eat(self.current_token.clone());
            // TODO clone here
            let right = self.parse_expr();
            ASTNode::BinaryOp(Box::new(left), op, Box::new(right))
        } else {
            left
        }
    }

    fn parse_term(&mut self) -> ASTNode {
        let left = self.parse_factor();
        if self.current_token == Token::Multiply || self.current_token == Token::Divide {
            let op = self.current_token.clone();
            self.eat(self.current_token.clone());
            let right = self.parse_term();
            ASTNode::BinaryOp(Box::new(left), op, Box::new(right))
        } else {
            left
        }
    }

    fn parse_factor(&mut self) -> ASTNode {
        let mut node = match &self.current_token {
            Token::Number(val) => {
                let num = *val;
                self.eat(Token::Number(num));
                ASTNode::Number(num)
            }
            Token::String(val) => {
                let s = val.clone();
                self.eat(Token::String(s.clone()));
                ASTNode::String(s)
            }
            Token::Boolean(val) => {
                let b = *val;
                self.eat(Token::Boolean(b));
                ASTNode::Boolean(b)
            }
            Token::Identifier(var_name) => {
                let name = var_name.clone();
                self.eat(Token::Identifier(var_name.clone()));
                if self.symbol_table.contains_key(&name) {
                    ASTNode::Identifier(name)
                } else {
                    panic!("Variable not declared: {}", name);
                }
            }
            Token::Null => {
                self.eat(Token::Null);
                ASTNode::Null
            }
            Token::LParen => {
                self.eat(Token::LParen);
                let expr = self.parse_expr();
                self.eat(Token::RParen);
                expr
            }
            Token::Type => {
                self.eat(Token::Type);
                self.eat(Token::LParen);
                let expr = self.parse_expr();
                self.eat(Token::RParen);
                ASTNode::Type(Box::new(expr))
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        };

        
        if self.current_token == Token::LBracket {
            self.eat(Token::LBracket);
            let index = self.parse_expr();
            self.eat(Token::RBracket);
            node = ASTNode::Index(Box::new(node), Box::new(index));
        }

        node
    }
}