#ifndef PARSER_H
#define PARSER_H

#include "lexer.h"
#include "ast.h"
#include <memory>
#include <vector>

static int CurTok;
int getNextToken(); // Defined in lexer.

// Forward declarations for recursive parsing.
std::unique_ptr<ExprAST> ParseExpression();
std::unique_ptr<ExprAST> ParsePrimary();
std::unique_ptr<ExprAST> ParseBinaryOpRHS(int ExprPrec, std::unique_ptr<ExprAST> LHS);

// Parse number literals.
std::unique_ptr<ExprAST> ParseNumberExpr() {
    auto Result = std::make_unique<NumberExprAST>(NumVal);
    getNextToken(); // Consume the number.
    return std::move(Result);
}

// Parse identifier or function calls.
std::unique_ptr<ExprAST> ParseIdentifierExpr() {
    std::string IdName = IdentifierStr;
    getNextToken(); // Eat the identifier.

    if (CurTok != '(') // Simple variable reference.
        return std::make_unique<VariableExprAST>(IdName);

    // Parse function call.
    getNextToken(); // Eat '('.
    std::vector<std::unique_ptr<ExprAST>> Args;
    if (CurTok != ')') {
        while (true) {
            if (auto Arg = ParseExpression())
                Args.push_back(std::move(Arg));
            else
                return nullptr;

            if (CurTok == ')')
                break;

            if (CurTok != ',')
                return nullptr; // Error: Expected ',' or ')'.
            getNextToken();
        }
    }

    getNextToken(); // Eat ')'.
    return std::make_unique<CallExprAST>(IdName, std::move(Args));
}

// Parse expressions.
std::unique_ptr<ExprAST> ParseExpression() {
    auto LHS = ParsePrimary();
    if (!LHS)
        return nullptr;
    return ParseBinaryOpRHS(0, std::move(LHS));
}

// Parse primary expressions (numbers, identifiers).
std::unique_ptr<ExprAST> ParsePrimary() {
    switch (CurTok) {
    case tok_identifier:
        return ParseIdentifierExpr();
    case tok_number:
        return ParseNumberExpr();
    default:
        return nullptr; // Error handling.
    }
}

#endif
