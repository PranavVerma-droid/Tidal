use crate::lexer::{Lexer, Token};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    Null,
}

#[derive(Debug)]
pub enum ASTNode {
    Number(i32),
    Null,
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
    Print(Box<ASTNode>),
    Var(String, Option<Box<ASTNode>>, bool), // String: variable name, Option<ASTNode>: value (None if not initialized), bool: is_mutable
    Assign(String, Box<ASTNode>),
    Identifier(String),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    symbol_table: HashMap<String, bool>, // is_mutable
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
        }
    }

    fn parse_var_decl(&mut self) -> ASTNode {
        let is_mutable = self.current_token == Token::Var;
        self.eat(if is_mutable { Token::Var } else { Token::NoVar });
        
        if let Token::Identifier(var_name) = &self.current_token {
            let name = var_name.clone();
            self.eat(Token::Identifier(var_name.clone()));
            
            if self.symbol_table.contains_key(&name) {
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
        match &self.current_token {
            Token::Number(val) => {
                let num = *val;
                self.eat(Token::Number(num));
                ASTNode::Number(num)
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
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }
}