// src/codegen.h
#pragma once

#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/Module.h>

class CodeGen {
public:
    static void initialize();
    static llvm::LLVMContext& getContext();
    static llvm::IRBuilder<>& getBuilder();
    static llvm::Module& getModule();

private:
    static std::unique_ptr<llvm::LLVMContext> TheContext;
    static std::unique_ptr<llvm::IRBuilder<>> Builder;
    static std::unique_ptr<llvm::Module> TheModule;
};