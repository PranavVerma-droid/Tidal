#ifndef LEXER_H
#define LEXER_H

#include <string>
#include <cctype>
#include <iostream>

enum Token {
    tok_eof = -1,
    tok_def = -2,
    tok_extern = -3,
    tok_identifier = -4,
    tok_number = -5
};

static std::string IdentifierStr; // For identifiers
static double NumVal;             // For number literals

// Lexer: Reads input and returns the next token.
int getNextToken() {
    static int LastChar = ' ';

    // Skip whitespace.
    while (isspace(LastChar))
        LastChar = getchar();

    // Identifier: [a-zA-Z][a-zA-Z0-9]*
    if (isalpha(LastChar)) {
        IdentifierStr = LastChar;
        while (isalnum((LastChar = getchar())))
            IdentifierStr += LastChar;
        if (IdentifierStr == "def")
            return tok_def;
        if (IdentifierStr == "extern")
            return tok_extern;
        return tok_identifier;
    }

    // Number: [0-9.]+
    if (isdigit(LastChar) || LastChar == '.') {
        std::string NumStr;
        do {
            NumStr += LastChar;
            LastChar = getchar();
        } while (isdigit(LastChar) || LastChar == '.');
        NumVal = strtod(NumStr.c_str(), nullptr);
        return tok_number;
    }

    // Handle comments starting with `#`.
    if (LastChar == '#') {
        do LastChar = getchar();
        while (LastChar != EOF && LastChar != '\n' && LastChar != '\r');
        if (LastChar != EOF)
            return getNextToken();
    }

    // End of file.
    if (LastChar == EOF)
        return tok_eof;

    // Otherwise, return the character as its ASCII value.
    int ThisChar = LastChar;
    LastChar = getchar();
    return ThisChar;
}

#endif
