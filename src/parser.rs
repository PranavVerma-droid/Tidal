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
    Function(String, Vec<String>, Vec<ASTNode>),  
    ReturnValue(Box<Value>),
}

#[derive(Debug, Clone)]
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
    FunctionDecl(String, Vec<String>, Vec<ASTNode>),  // name, params, body
    FunctionCall(String, Vec<ASTNode>),  // name, arguments
    Input(Box<ASTNode>),
    LenCall(Box<ASTNode>),
    DelCall(Box<ASTNode>),
    Return(Option<Box<ASTNode>>),
}

#[derive(Clone)]
struct Scope {
    variables: HashMap<String, bool>,
    is_function: bool,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    scopes: Vec<Scope>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token().unwrap();
        let mut parser = Parser {
            lexer,
            current_token,
            scopes: Vec::new(),
        };
        parser.push_scope(false);
        parser
    }

    fn push_scope(&mut self, is_function: bool) {
        self.scopes.push(Scope {
            variables: HashMap::new(),
            is_function,
        });
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn current_scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn remove_from_sym_table(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.variables.remove(name);
        }
    }

    fn is_variable_declared(&self, name: &str) -> bool {
        if self.current_scope().is_function {
            return self.current_scope().variables.contains_key(name);
        }
        
        for scope in self.scopes.iter().rev() {
            if scope.variables.contains_key(name) {
                return true;
            }
        }
        false
    }

    fn eat(&mut self, token: Token) -> Result<(), Error> {
        if self.current_token == token {
            self.current_token = self.lexer.next_token()?;
            Ok(())
        } else {
            Err(Error::ParserError(format!("Unexpected token: {:?}, expected: {:?} at line {}", self.current_token, token, self.lexer.line)))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<ASTNode>, Error> {
        let mut ast_nodes = Vec::new();
        while self.current_token != Token::EOF {
            ast_nodes.push(self.parse_statement()?);
        }
        Ok(ast_nodes)
    }


    fn parse_function_decl(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::Func)?;
        
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            self.eat(Token::Identifier(name.clone()))?;
            name
        } else {
            return Err(Error::ParserError("Expected function name".to_string()));
        };

        if Self::is_keyword(&name) {
            return Err(Error::SyntaxError(format!("Cannot use keyword '{}' as function name", name)));
        }

        self.eat(Token::LParen)?;
        
        let mut params = Vec::new();
        while self.current_token != Token::RParen {
            if let Token::Identifier(param) = self.current_token.clone() {
                params.push(param.clone());
                self.eat(Token::Identifier(param))?;
                
                if self.current_token == Token::Comma {
                    self.eat(Token::Comma)?;
                }
            } else {
                return Err(Error::ParserError("Expected parameter name".to_string()));
            }
        }
        
        self.eat(Token::RParen)?;
        self.eat(Token::LBrace)?;
        
        self.push_scope(true);
        
        let mut body = Vec::new();
        while self.current_token != Token::RBrace {
            body.push(self.parse_statement()?);
        }
        
        self.pop_scope();
        
        self.eat(Token::RBrace)?;
        
        Ok(ASTNode::FunctionDecl(name, params, body))
    }

    fn parse_return(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::Return)?;
        
        let expr = if self.current_token != Token::Semicolon {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        
        self.eat(Token::Semicolon)?;
        
        Ok(ASTNode::Return(expr))
    }
    fn parse_statement(&mut self) -> Result<ASTNode, Error> {
        match &self.current_token {
            Token::Var | Token::NoVar => self.parse_var_decl(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if_statement(),
            Token::For => self.parse_for_loop(),
            Token::Break => self.parse_break(),
            Token::Continue => self.parse_continue(),
            Token::While => self.parse_while_loop(),
            Token::Type => self.parse_type(),
            Token::Func => self.parse_function_decl(),
            Token::Return => self.parse_return(),
            Token::Del => {
                let node = self.parse_del()?;
                self.eat(Token::Semicolon)?;
                Ok(node)
            },
            Token::Identifier(name) => {
                let name = name.clone();
                self.eat(Token::Identifier(name.clone()))?;
                
                match &self.current_token {
                    Token::LParen => {
                        // function call
                        self.eat(Token::LParen)?;
                        let mut args = Vec::new();
                        
                        if self.current_token != Token::RParen {
                            loop {
                                args.push(self.parse_expr()?);
                                if self.current_token == Token::Comma {
                                    self.eat(Token::Comma)?;
                                } else {
                                    break;
                                }
                            }
                        }
                        
                        self.eat(Token::RParen)?;
                        self.eat(Token::Semicolon)?; 
                        
                        Ok(ASTNode::FunctionCall(name, args))
                    },
                    Token::Assign | Token::LBracket => {
                        let node = ASTNode::Identifier(name);
                        self.parse_assign_stmt_with_node(node)
                    },
                    _ => Err(Error::ParserError(format!(
                        "Unexpected token after identifier: {:?} at line {}", 
                        self.current_token, 
                        self.lexer.line
                    ))),
                }
            },
            _ => Err(Error::ParserError(format!(
                "Unexpected token in statement: {:?} at line {}", 
                self.current_token, 
                self.lexer.line
            ))),
        }
    }
    fn is_keyword(name: &str) -> bool {
        matches!(name, 
            "var" | "novar" | "print" | "type" | "if" | "elif" | "else" | 
            "null" | "true" | "false" | "for" | "while" | "break" | "continue" |
            "int" | "str" | "float" | "bool" | "func" | "return"
        )
    }

    fn parse_del(&mut self) -> Result<ASTNode, Error> {
        self.eat(Token::Del)?;
        self.eat(Token::LParen)?;
        if let Token::Identifier(name) = self.current_token.clone() {
            self.remove_from_sym_table(&name);
        }
        let expr = self.parse_expr()?;
        self.eat(Token::RParen)?;
        Ok(ASTNode::DelCall(Box::new(expr)))
    }

    fn parse_assign_stmt_with_node(&mut self, left: ASTNode) -> Result<ASTNode, Error> {
        if let ASTNode::Identifier(name) = &left { //check for array first
            if self.current_token == Token::LBracket {
                self.eat(Token::LBracket)?;
                let index = self.parse_expr()?;
                self.eat(Token::RBracket)?;
                self.eat(Token::Assign)?;
                let value = self.parse_expr()?;
                self.eat(Token::Semicolon)?;
                return Ok(ASTNode::IndexAssign(
                    Box::new(ASTNode::Identifier(name.clone())),
                    Box::new(index),
                    Box::new(value)
                ));
            }
        }
        match left { //then function.
            ASTNode::Identifier(name) => {
                self.eat(Token::Assign)?;
                let value = self.parse_expr()?;
                self.eat(Token::Semicolon)?;
                Ok(ASTNode::Assign(name, Box::new(value)))
            },
            _ => Err(Error::ParserError("Invalid assignment target".to_string()))
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
            Token::Input => {
                self.eat(Token::Input)?;
                self.eat(Token::LParen)?;
                let prompt = self.parse_expr()?;
                self.eat(Token::RParen)?;
                Ok(ASTNode::Input(Box::new(prompt)))
            },
            Token::Len => {
                self.eat(Token::Len)?;
                self.eat(Token::LParen)?;
                let expr = self.parse_expr()?;
                self.eat(Token::RParen)?;
                Ok(ASTNode::LenCall(Box::new(expr)))
            },
            Token::Del => {
                self.eat(Token::Del)?;
                self.eat(Token::LParen)?;
                let expr = self.parse_expr()?;
                self.eat(Token::RParen)?;
                Ok(ASTNode::DelCall(Box::new(expr)))
            },
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
            },
            Token::LParen => {
                self.eat(Token::LParen)?;
                let expr = self.parse_expr()?;
                self.eat(Token::RParen)?;
                Ok(expr)
            },
            Token::LBracket => self.parse_array_literal(),
            Token::Identifier(_) | Token::String(_) | Token::Boolean(_) | Token::Null | Token::TypeLiteral(_) | Token::TypeCast(_) | Token::Type => {
                self.parse_primary()
            },
            _ => Err(Error::ParserError(format!("Unexpected token in factor: {:?} at line {}", self.current_token, self.lexer.line))),
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
            Token::Del => {
                self.eat(Token::Del)?;
                self.eat(Token::LParen)?;
                let expr = self.parse_expr()?;
                self.eat(Token::RParen)?;
                ASTNode::DelCall(Box::new(expr))
            }
            Token::Identifier(var_name) => {
                let name = var_name.clone();
                self.eat(Token::Identifier(name.clone()))?;
                
                // check for function call
                if self.current_token == Token::LParen {
                    self.eat(Token::LParen)?;
                    let mut args = Vec::new();
                    
                    if self.current_token != Token::RParen {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.current_token == Token::Comma {
                                self.eat(Token::Comma)?;
                            } else {
                                break;
                            }
                        }
                    }
                    
                    self.eat(Token::RParen)?;
                    ASTNode::FunctionCall(name, args)
                } else {
                    ASTNode::Identifier(name)
                }
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
            _ => return Err(Error::ParserError(format!("Unexpected token in primary: {:?} at line {}", self.current_token, self.lexer.line))),
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
            _ => return Err(Error::ParserError(format!("Expected var or novar at line {}", self.lexer.line))),
        };
        self.eat(self.current_token.clone())?;

        let name = if let Token::Identifier(ident) = self.current_token.clone() {
            self.eat(Token::Identifier(ident.clone()))?;
            ident
        } else {
            return Err(Error::ParserError(format!("Expected identifier in variable declaration at line {}", self.lexer.line)));
        };

        if self.is_variable_declared(&name) {
            return Err(Error::VariableAlreadyDeclared(format!("Variable '{}' has already been declared at line {}", name, self.lexer.line)));
        }

        self.current_scope_mut().variables.insert(name.clone(), is_mutable);

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
            return Err(Error::ParserError(format!("Expected identifier in assignment at line {}", self.lexer.line)));
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
