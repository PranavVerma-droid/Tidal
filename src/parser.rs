use crate::lexer::{Lexer, Token};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ASTNode {
    Number(i32),
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
    Print(Box<ASTNode>),
    Var(String, Box<ASTNode>),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    symbol_table: HashMap<String, i32>, // Symbol table to store variable values
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
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
            panic!("Unexpected token: {:?}", self.current_token);
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
            Token::Var => self.parse_var_decl(),
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
            let value = self.evaluate_expr(&expr); // Evaluate expression and store the result
            self.symbol_table.insert(name.clone(), value); // Store variable in symbol table
            self.eat(Token::Semicolon);
            ASTNode::Var(name, Box::new(expr))
        } else {
            panic!("Expected variable name");
        }
    }

    fn parse_var_decl(&mut self) -> ASTNode {
        self.eat(Token::Var);
        if let Token::Identifier(var_name) = &self.current_token {
            let name = var_name.clone();
            self.eat(Token::Identifier(var_name.clone()));
            self.eat(Token::Assign);
            let expr = self.parse_expr();
            let value = self.evaluate_expr(&expr); // Evaluate expression and store the result
            self.symbol_table.insert(name.clone(), value); // Store variable in symbol table
            self.eat(Token::Semicolon);
            ASTNode::Var(name, Box::new(expr))
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
            Token::Identifier(_) => self.parse_variable(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_variable(&mut self) -> ASTNode {
        if let Token::Identifier(var_name) = &self.current_token {
            let name = var_name.clone();
            self.eat(Token::Identifier(var_name.clone()));
            let value = self.symbol_table.get(&name).cloned().unwrap_or(0);
            ASTNode::Number(value)
        } else {
            panic!("Expected variable name");
        }
    }

    fn evaluate_expr(&mut self, node: &ASTNode) -> i32 {
        match node {
            ASTNode::Number(value) => *value,
            ASTNode::BinaryOp(left, token, right) => {
                let left_val = self.evaluate_expr(left);
                let right_val = self.evaluate_expr(right);
                match token {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Multiply => left_val * right_val,
                    Token::Divide => left_val / right_val,
                    _ => panic!("Unexpected binary operator: {:?}", token),
                }
            }
            _ => panic!("Unexpected AST node: {:?}", node),
        }
    }
}
