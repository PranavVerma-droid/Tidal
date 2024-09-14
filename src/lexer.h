// src/lexer.h
#pragma once

#include <string>
#include <vector>

enum class TokenType {
    INTEGER,
    PLUS,
    MINUS,
    EOF_TOKEN
};

struct Token {
    TokenType type;
    std::string value;
};

class Lexer {
public:
    Lexer(const std::string& input);
    Token getNextToken();

private:
    std::string m_input;
    size_t m_position;

    void skipWhitespace();  // Add this line
};