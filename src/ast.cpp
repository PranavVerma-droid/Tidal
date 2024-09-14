// src/ast.cpp
#include "ast.h"
#include "codegen.h"

IntegerNode::IntegerNode(int value) : m_value(value) {}

llvm::Value* IntegerNode::codegen() {
    return llvm::ConstantInt::get(CodeGen::getContext(), llvm::APInt(32, m_value));
}

BinaryOpNode::BinaryOpNode(char op, std::unique_ptr<ASTNode> left, std::unique_ptr<ASTNode> right)
    : m_op(op), m_left(std::move(left)), m_right(std::move(right)) {}

llvm::Value* BinaryOpNode::codegen() {
    llvm::Value* L = m_left->codegen();
    llvm::Value* R = m_right->codegen();

    switch (m_op) {
        case '+':
            return CodeGen::getBuilder().CreateAdd(L, R, "addtmp");
        case '-':
            return CodeGen::getBuilder().CreateSub(L, R, "subtmp");
        default:
            throw std::runtime_error("Invalid binary operator");
    }
}