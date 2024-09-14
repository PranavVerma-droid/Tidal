// src/parser.cpp
#include "parser.h"
#include <stdexcept>

Parser::Parser(Lexer& lexer) : m_lexer(lexer) {
    advance();
}

void Parser::advance() {
    m_currentToken = m_lexer.getNextToken();
}

std::unique_ptr<ASTNode> Parser::parse() {
    return parseExpression();
}

std::unique_ptr<ASTNode> Parser::parseExpression() {
    auto node = parseTerm();

    while (m_currentToken.type == TokenType::PLUS || m_currentToken.type == TokenType::MINUS) {
        Token op = m_currentToken;
        advance();
        auto right = parseTerm();
        node = std::make_unique<BinaryOpNode>(op.value[0], std::move(node), std::move(right));
    }

    return node;
}

std::unique_ptr<ASTNode> Parser::parseTerm() {
    return parseFactor();
}

std::unique_ptr<ASTNode> Parser::parseFactor() {
    if (m_currentToken.type == TokenType::INTEGER) {
        auto node = std::make_unique<IntegerNode>(std::stoi(m_currentToken.value));
        advance();
        return node;
    }

    throw std::runtime_error("Unexpected token");
}