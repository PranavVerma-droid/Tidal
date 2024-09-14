// src/lexer.cpp
#include "lexer.h"
#include <stdexcept>
#include <sstream>

Lexer::Lexer(const std::string& input) : m_input(input), m_position(0) {}

void Lexer::skipWhitespace() {
    while (m_position < m_input.length() && std::isspace(m_input[m_position])) {
        m_position++;
    }
}

Token Lexer::getNextToken() {
    skipWhitespace();

    if (m_position >= m_input.length()) {
        return {TokenType::EOF_TOKEN, ""};
    }

    char currentChar = m_input[m_position];

    if (std::isdigit(currentChar)) {
        std::string number;
        while (m_position < m_input.length() && std::isdigit(m_input[m_position])) {
            number += m_input[m_position];
            m_position++;
        }
        return {TokenType::INTEGER, number};
    }

    if (currentChar == '+') {
        m_position++;
        return {TokenType::PLUS, "+"};
    }

    if (currentChar == '-') {
        m_position++;
        return {TokenType::MINUS, "-"};
    }

    std::ostringstream errorMsg;
    errorMsg << "Invalid character '" << currentChar << "' at position " << m_position;
    throw std::runtime_error(errorMsg.str());
}