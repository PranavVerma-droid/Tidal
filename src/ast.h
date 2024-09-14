#ifndef AST_H
#define AST_H

#include "llvm/IR/Value.h"
#include <string>
#include <vector>  // <-- Add this to include std::vector.
#include <memory>

// Base class for all expression nodes.
class ExprAST {
public:
    virtual ~ExprAST() = default;

    // Declare codegen method that derived classes must implement.
    virtual llvm::Value* codegen() = 0;
};

// Expression class for numeric literals like "1.0".
class NumberExprAST : public ExprAST {
    double Val;
public:
    NumberExprAST(double Val) : Val(Val) {}

    llvm::Value* codegen() override;
};

// Expression class for referencing a variable, like "a".
class VariableExprAST : public ExprAST {
    std::string Name;
public:
    VariableExprAST(const std::string &Name) : Name(Name) {}

    llvm::Value* codegen() override;
};

// Expression class for function calls.
class CallExprAST : public ExprAST {
    std::string Callee;
    std::vector<std::unique_ptr<ExprAST>> Args;  // <-- Ensure this is std::vector.
public:
    CallExprAST(const std::string &Callee, std::vector<std::unique_ptr<ExprAST>> Args)
        : Callee(Callee), Args(std::move(Args)) {}  // <-- Args is now a vector.

    llvm::Value* codegen() override;
};

#endif
