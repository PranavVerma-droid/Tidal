// src/parser.h
#pragma once

#include "lexer.h"
#include "ast.h"
#include <memory>

class Parser {
public:
    Parser(Lexer& lexer);
    std::unique_ptr<ASTNode> parse();

private:
    Lexer& m_lexer;
    Token m_currentToken;

    void advance();
    std::unique_ptr<ASTNode> parseExpression();
    std::unique_ptr<ASTNode> parseTerm();
    std::unique_ptr<ASTNode> parseFactor();
};