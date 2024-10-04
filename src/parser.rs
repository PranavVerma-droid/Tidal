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
    TypeLiteral(String),
    If(Box<ASTNode>, Vec<ASTNode>, Vec<(ASTNode, Vec<ASTNode>)>, Option<Vec<ASTNode>>),
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
            Token::Print => self.parse_print(),
            Token::If => self.parse_if_statement(),
            _ => panic!("Unexpected token in statement: {:?}", self.current_token),
        }
    }

    fn parse_if_statement(&mut self) -> ASTNode {
        self.eat(Token::If);
        self.eat(Token::LParen);
        let condition = self.parse_expr();
        self.eat(Token::RParen);
        self.eat(Token::LBrace);
        let if_block = self.parse_block();
        self.eat(Token::RBrace);

        let mut elif_blocks = Vec::new();
        let mut else_block = None;

        while self.current_token == Token::Elif {
            self.eat(Token::Elif);
            self.eat(Token::LParen);
            let elif_condition = self.parse_expr();
            self.eat(Token::RParen);
            self.eat(Token::LBrace);
            let elif_statements = self.parse_block();
            self.eat(Token::RBrace);
            elif_blocks.push((elif_condition, elif_statements));
        }

        if self.current_token == Token::Else {
            self.eat(Token::Else);
            self.eat(Token::LBrace);
            else_block = Some(self.parse_block());
            self.eat(Token::RBrace);
        }

        ASTNode::If(Box::new(condition), if_block, elif_blocks, else_block)
    }

    fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut statements = Vec::new();
        while self.current_token != Token::RBrace {
            statements.push(self.parse_statement());
        }
        statements
    }

    fn parse_expr(&mut self) -> ASTNode {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> ASTNode {
        let mut node = self.parse_arithmetic();
    
        loop {
            match &self.current_token {
                Token::Equal | Token::NotEqual | Token::Greater | Token::Less | Token::GreaterEqual | Token::LessEqual => {
                    let op = self.current_token.clone();
                    self.eat(op.clone());
                    let right = self.parse_arithmetic();
                    node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
                }
                _ => break,
            }
        }
    
        node
    }

    fn parse_arithmetic(&mut self) -> ASTNode {
        let mut node = self.parse_term();

        loop {
            match &self.current_token {
                Token::Plus | Token::Minus => {
                    let op = self.current_token.clone();
                    self.eat(op.clone());
                    let right = self.parse_term();
                    node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
                }
                _ => break,
            }
        }

        node
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();

        loop {
            match &self.current_token {
                Token::Multiply | Token::Divide => {
                    let op = self.current_token.clone();
                    self.eat(op.clone());
                    let right = self.parse_factor();
                    node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
                }
                _ => break,
            }
        }

        node
    }

    fn parse_factor(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Minus => {
                self.eat(Token::Minus);
                let factor = self.parse_factor();
                ASTNode::BinaryOp(Box::new(ASTNode::Number(0)), Token::Minus, Box::new(factor))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> ASTNode {
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
                self.eat(Token::Identifier(name.clone()));
                ASTNode::Identifier(name)
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
            Token::TypeLiteral(type_name) => {
                let name = type_name.clone();
                self.eat(Token::TypeLiteral(name.clone()));
                ASTNode::TypeLiteral(name)
            }
            _ => panic!("Unexpected token in primary: {:?}", self.current_token),
        };
    
        while self.current_token == Token::LBracket {
            self.eat(Token::LBracket);
            let index = self.parse_expr();
            self.eat(Token::RBracket);
            node = ASTNode::Index(Box::new(node), Box::new(index));
        }
    
        node
    }
    fn parse_var_decl(&mut self) -> ASTNode {
        let is_mutable = self.current_token == Token::Var;
        self.eat(if is_mutable { Token::Var } else { Token::NoVar });
        
        if let Token::Identifier(var_name) = &self.current_token {
            let name = var_name.clone();
            self.eat(Token::Identifier(name.clone()));
            
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
            panic!("Expected variable name, got: {:?}", self.current_token);
        }
    }

    fn parse_assign_stmt(&mut self) -> ASTNode {
        if let Token::Identifier(var_name) = &self.current_token {
            let name = var_name.clone();
            self.eat(Token::Identifier(name.clone()));
            self.eat(Token::Assign);
            let expr = self.parse_expr();
            
            if !self.symbol_table.contains_key(&name) {
                //sym bio table not dec
                panic!("Variable not declared: {}", name);
            }
            
            self.eat(Token::Semicolon);
            ASTNode::Assign(name, Box::new(expr))
        } else {
            panic!("Expected variable name, got: {:?}", self.current_token);
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
}