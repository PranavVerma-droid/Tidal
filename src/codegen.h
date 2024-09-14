#ifndef CODEGEN_H
#define CODEGEN_H

#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Verifier.h"
#include "ast.h"
#include <map>
#include <string>

static std::map<std::string, llvm::Value*> NamedValues;



static llvm::LLVMContext TheContext;
static llvm::IRBuilder<> Builder(TheContext);
static std::unique_ptr<llvm::Module> TheModule;

// Utility function to log errors.
llvm::Value *LogErrorV(const char *Str) {
    std::cerr << "Error: " << Str << std::endl;
    return nullptr;
}

// Generate LLVM IR for NumberExprAST (numeric literals).
llvm::Value *NumberExprAST::codegen() {
    return llvm::ConstantFP::get(TheContext, llvm::APFloat(Val));
}

// Generate LLVM IR for VariableExprAST (variables).
llvm::Value *VariableExprAST::codegen() {
    llvm::Value *V = NamedValues[Name];
    if (!V)
        return LogErrorV("Unknown variable name");
    return V;
}

#endif
