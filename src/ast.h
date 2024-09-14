// src/ast.h
#pragma once

#include <llvm/IR/Value.h>

class ASTNode {
public:
    virtual ~ASTNode() = default;
    virtual llvm::Value* codegen() = 0;
};

class IntegerNode : public ASTNode {
public:
    IntegerNode(int value);
    llvm::Value* codegen() override;

private:
    int m_value;
};

class BinaryOpNode : public ASTNode {
public:
    BinaryOpNode(char op, std::unique_ptr<ASTNode> left, std::unique_ptr<ASTNode> right);
    llvm::Value* codegen() override;

private:
    char m_op;
    std::unique_ptr<ASTNode> m_left;
    std::unique_ptr<ASTNode> m_right;
};