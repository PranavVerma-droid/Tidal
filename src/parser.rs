use crate::lexer::{Lexer, Token};
use crate::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    String(String),
    Boolean(bool),
    Float(f64), 
    Null,
    Type(String),
    Break,
    Continue,
    Array(Vec<Value>),
}

#[derive(Debug)]
pub enum ASTNode {
    Number(i32),
    String(String),
    Boolean(bool),
    Float(f64), 
    Null,
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
    Print(Box<ASTNode>),
    Var(String, Option<Box<ASTNode>>, bool),
    Assign(String, Box<ASTNode>),
    UnaryOp(Token, Box<ASTNode>),
    Identifier(String),
    Index(Box<ASTNode>, Box<ASTNode>),
    IndexAssign(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>),
    Type(Box<ASTNode>),
    TypeLiteral(String),
    TypeCast(String, Box<ASTNode>),
    If(Box<ASTNode>, Vec<ASTNode>, Vec<(ASTNode, Vec<ASTNode>)>, Option<Vec<ASTNode>>),
    For(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>, Vec<ASTNode>),
    While(Box<ASTNode>, Vec<ASTNode>),
    Array(Vec<ASTNode>),
    Break,
    Continue,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    symbol_table: HashMap<String, bool>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token().unwrap();
        Parser {
            lexer,
            current_token,
            symbol_table: HashMap::new(),
        }
    }

    fn eat(&mut self, token: Token) -> Result<(), Error> {
        if self.current_token == token {
            self.current_token = self.lexer.next_token()?;
            Ok(())
        } else {
            Err(Error::SyntaxError(format!("Unexpected token: {:?}, expected: {:?} at line {}", self.current_token, token, self.lexer.line)))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<ASTNode>, Error> {
        let mut ast_nodes = Vec::new();
        while self.current_token != Token::EOF {
            ast_nodes.push(self.parse_statement()?);
        }
        Ok(ast_nodes)
    }

    fn parse_statement(&mut self) -> Result<ASTNode, Error> {
        match &self.current_token {
            Token::Var | Token::NoVar => self.parse_var_decl(),
            Token::Identifier(_) => self.parse_assign_stmt(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if_statement(),
            Token::For => self.parse_for_loop(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::While => self.parse_while_loop(),
            Token::Type => self.parse_type(),
            _ => Err(Error::SyntaxError(format!("Unexpected token in statement: {:?} at line {}", self.current_token, self.lexer.line))),
        }
    }

    fn parse_type(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::Type)?;
        self.eat(Token::LParen)?;
        let expr = self.parse_expr()?;
        self.eat(Token::RParen)?;
        self.eat(Token::Semicolon)?;
        Ok(ASTNode::Type(Box::new(expr)))
    }

    fn parse_while_loop(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::While)?;
        self.eat(Token::LParen)?;
        let condition = self.parse_expr()?;
        self.eat(Token::RParen)?;
        self.eat(Token::LBrace)?;
        let body = self.parse_block()?;
        self.eat(Token::RBrace)?;

        Ok(ASTNode::While(Box::new(condition), body))
    }

    fn parse_if_statement(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::If)?;
        self.eat(Token::LParen)?;
        let condition = self.parse_expr()?;
        self.eat(Token::RParen)?;
        self.eat(Token::LBrace)?;
        let if_block = self.parse_block()?;
        self.eat(Token::RBrace)?;

        let mut elif_blocks = Vec::new();
        let mut else_block = None;

        while self.current_token == Token::Elif {
            self.eat(Token::Elif)?;
            self.eat(Token::LParen)?;
            let elif_condition = self.parse_expr()?;
            self.eat(Token::RParen)?;
            self.eat(Token::LBrace)?;
            let elif_statements = self.parse_block()?;
            self.eat(Token::RBrace)?;
            elif_blocks.push((elif_condition, elif_statements));
        }

        if self.current_token == Token::Else {
            self.eat(Token::Else)?;
            self.eat(Token::LBrace)?;
            else_block = Some(self.parse_block()?);
            self.eat(Token::RBrace)?;
        }

        Ok(ASTNode::If(Box::new(condition), if_block, elif_blocks, else_block))
    }

    fn parse_for_loop(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::For)?;
        self.eat(Token::LParen)?;

        let init = if let Token::Var | Token::NoVar = self.current_token {
            self.parse_var_decl()?
        } else {
            self.parse_assign_stmt()?
        };

        let condition = self.parse_expr()?;
        self.eat(Token::Semicolon)?;

        let update = self.parse_assign_stmt()?;
        self.eat(Token::RParen)?;

        self.eat(Token::LBrace)?;
        let body = self.parse_block()?;
        self.eat(Token::RBrace)?;

        Ok(ASTNode::For(Box::new(init), Box::new(condition), Box::new(update), body))
    }

    fn parse_break(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::Break)?;
        self.eat(Token::Semicolon)?;
        Ok(ASTNode::Break)
    }

    fn parse_continue(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::Continue)?;
        self.eat(Token::Semicolon)?;
        Ok(ASTNode::Continue)
    }

    fn parse_block(&mut self) -> Result<Vec<ASTNode>, Error> {
        let mut statements = Vec::new();
        while self.current_token != Token::RBrace {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_expr(&mut self) -> Result<ASTNode, Error> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<ASTNode, Error> {
        let mut node = self.parse_logical_and()?;

        while self.current_token == Token::Or {
            let op = self.current_token.clone();
            self.eat(Token::Or)?;
            let right = self.parse_logical_and()?;
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
        }

        Ok(node)
    }

    fn parse_logical_and(&mut self) -> Result<ASTNode, Error> {
        let mut node = self.parse_comparison()?;

        while self.current_token == Token::And {
            let op = self.current_token.clone();
            self.eat(Token::And)?;
            let right = self.parse_comparison()?;
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
        }

        Ok(node)
    }

    fn parse_comparison(&mut self) -> Result<ASTNode, Error> {
        let mut node = self.parse_arithmetic()?;

        loop {
            match &self.current_token {
                Token::Equal | Token::NotEqual | Token::Greater | Token::Less | Token::GreaterEqual | Token::LessEqual => {
                    let op = self.current_token.clone();
                    self.eat(op.clone())?;
                    let right = self.parse_arithmetic()?;
                    node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_arithmetic(&mut self) -> Result<ASTNode, Error> {
        let mut node = self.parse_term()?;

        loop {
            match &self.current_token {
                Token::Plus | Token::Minus => {
                    let op = self.current_token.clone();
                    self.eat(op.clone())?;
                    let right = self.parse_term()?;
                    node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> Result<ASTNode, Error> {
        let mut node = self.parse_power()?;

        loop {
            match &self.current_token {
                Token::Multiply => {
                    self.eat(Token::Multiply)?;
                    let right = self.parse_power()?;
                    node = ASTNode::BinaryOp(Box::new(node), Token::Multiply, Box::new(right));
                }
                Token::Divide => {
                    self.eat(Token::Divide)?;
                    if self.current_token == Token::Divide {
                        self.eat(Token::Divide)?;
                        let right = self.parse_power()?;
                        node = ASTNode::BinaryOp(Box::new(node), Token::FloorDivide, Box::new(right));
                    } else {
                        let right = self.parse_power()?;
                        node = ASTNode::BinaryOp(Box::new(node), Token::Divide, Box::new(right));
                    }
                }
                Token::Modulus => {
                    self.eat(Token::Modulus)?;
                    let right = self.parse_power()?;
                    node = ASTNode::BinaryOp(Box::new(node), Token::Modulus, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_power(&mut self) -> Result<ASTNode, Error> {
        let mut node = self.parse_factor()?;

        while self.current_token == Token::Power {
            let op = self.current_token.clone();
            self.eat(Token::Power)?;
            let right = self.parse_factor()?;
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right));
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, Error> {
        match &self.current_token {
            Token::Minus => {
                self.eat(Token::Minus)?;
                let factor = self.parse_factor()?;
                Ok(ASTNode::BinaryOp(Box::new(ASTNode::Number(0)), Token::Minus, Box::new(factor)))
            }
            Token::Number(val) => {
                let num = *val;
                self.eat(Token::Number(num))?;
                Ok(ASTNode::Number(num))
            }
            Token::Not => {
                self.eat(Token::Not)?;
                let factor = self.parse_factor()?;
                Ok(ASTNode::UnaryOp(Token::Not, Box::new(factor)))
            },
            Token::Float(val) => {
                let num = *val;
                self.eat(Token::Float(num))?;
                Ok(ASTNode::Float(num))
            }
            Token::LParen => {
                self.eat(Token::LParen)?;
                let expr = self.parse_expr()?;
                self.eat(Token::RParen)?;
                Ok(expr)
            }
            Token::LBracket => self.parse_array_literal(),
            Token::Identifier(_) | Token::String(_) | Token::Boolean(_) | Token::Null | Token::TypeLiteral(_) | Token::TypeCast(_) | Token::Type => {
                self.parse_primary()
            }
            _ => Err(Error::SyntaxError(format!("Unexpected token in factor: {:?} at line {}", self.current_token, self.lexer.line))),
        }
    }

    fn parse_primary(&mut self) -> Result<ASTNode, Error> {
        let mut node = match &self.current_token {
            Token::Number(val) => {
                let num = *val;
                self.eat(Token::Number(num))?;
                ASTNode::Number(num)
            }
            Token::Float(val) => {
                let num = *val;
                self.eat(Token::Float(num))?;
                ASTNode::Float(num)
            }
            Token::String(val) => {
                let s = val.clone();
                self.eat(Token::String(s.clone()))?;
                ASTNode::String(s)
            }
            Token::Boolean(val) => {
                let b = *val;
                self.eat(Token::Boolean(b))?;
                ASTNode::Boolean(b)
            }
            Token::Identifier(var_name) => {
                let name = var_name.clone();
                self.eat(Token::Identifier(name.clone()))?;
                ASTNode::Identifier(name)
            }
            Token::TypeLiteral(type_name) => {
                let name = type_name.clone();
                self.eat(Token::TypeLiteral(name.clone()))?;
                ASTNode::TypeLiteral(name)
            }
            Token::TypeCast(type_name) => {
                self.parse_type_cast(type_name.clone())?
            }
            Token::Null => {
                self.eat(Token::Null)?;
                ASTNode::Null
            }
            Token::LParen => {
                self.eat(Token::LParen)?;
                let expr = self.parse_expr()?;
                self.eat(Token::RParen)?;
                expr
            }
            Token::Type => {
                self.eat(Token::Type)?;
                self.eat(Token::LParen)?;
                let expr = self.parse_expr()?;
                self.eat(Token::RParen)?;
                ASTNode::Type(Box::new(expr))
            }
            _ => return Err(Error::SyntaxError(format!("Unexpected token in primary: {:?} at line {}", self.current_token, self.lexer.line))),
        };
        while self.current_token == Token::LBracket {
            node = self.parse_index(node)?;
        }

        Ok(node)
    }

    fn parse_array_literal(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::LBracket)?;
        let mut elements = Vec::new();

        if self.current_token != Token::RBracket {
            loop {
                elements.push(self.parse_expr()?);
                if self.current_token == Token::Comma {
                    self.eat(Token::Comma)?;
                } else {
                    break;
                }
            }
        }

        self.eat(Token::RBracket)?;
        Ok(ASTNode::Array(elements))
    }

    fn parse_index(&mut self, expr: ASTNode) -> Result<ASTNode, Error> {
        self.eat(Token::LBracket)?;
        let index = self.parse_expr()?;
        self.eat(Token::RBracket)?;
        Ok(ASTNode::Index(Box::new(expr), Box::new(index)))
    }

    fn parse_type_cast(&mut self, type_name: String) -> Result<ASTNode, Error> {
        self.eat(Token::TypeCast(type_name.clone()))?;
        self.eat(Token::LParen)?;
        let expr = self.parse_expr()?;
        self.eat(Token::RParen)?;
        Ok(ASTNode::TypeCast(type_name, Box::new(expr)))
    }

    fn parse_var_decl(&mut self) -> Result<ASTNode, Error> {
        let is_mutable = match self.current_token {
            Token::Var => true,
            Token::NoVar => false,
            _ => return Err(Error::SyntaxError(format!("Expected var or novar at line {}", self.lexer.line))),
        };
        self.eat(self.current_token.clone())?;

        let name = if let Token::Identifier(ident) = self.current_token.clone() {
            self.eat(Token::Identifier(ident.clone()))?;
            ident
        } else {
            return Err(Error::SyntaxError(format!("Expected identifier in variable declaration at line {}", self.lexer.line)));
        };

        if self.symbol_table.contains_key(&name) {
            return Err(Error::VariableAlreadyDeclared(format!("Variable '{}' has already been declared at line {}", name, self.lexer.line)));
        }

        self.symbol_table.insert(name.clone(), is_mutable);

        let initializer = if self.current_token == Token::Assign {
            self.eat(Token::Assign)?;
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };

        self.eat(Token::Semicolon)?;
        Ok(ASTNode::Var(name, initializer, is_mutable))
    }

    fn parse_assign_stmt(&mut self) -> Result<ASTNode, Error> {
        let name = if let Token::Identifier(ident) = self.current_token.clone() {
            self.eat(Token::Identifier(ident.clone()))?;
            ident
        } else {
            return Err(Error::SyntaxError(format!("Expected identifier in assignment at line {}", self.lexer.line)));
        };

        let mut expr = ASTNode::Identifier(name.clone());
        if self.current_token == Token::LBracket {
            self.eat(Token::LBracket)?;
            let index = self.parse_expr()?;
            self.eat(Token::RBracket)?;
            expr = ASTNode::Index(Box::new(expr), Box::new(index));
        }

        self.eat(Token::Assign)?;
        let value = self.parse_expr()?;

        if self.current_token == Token::Semicolon {
            self.eat(Token::Semicolon)?;
        }

        match expr {
            ASTNode::Index(array, index) => Ok(ASTNode::IndexAssign(array, index, Box::new(value))),
            _ => Ok(ASTNode::Assign(name, Box::new(value))),
        }
    }

    fn parse_print(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::Print)?;
        self.eat(Token::LParen)?;
        let expr = self.parse_expr()?;
        self.eat(Token::RParen)?;
        self.eat(Token::Semicolon)?;
        Ok(ASTNode::Print(Box::new(expr)))
    }
}
