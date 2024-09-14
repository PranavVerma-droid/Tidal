// src/codegen.cpp
#include "codegen.h"

std::unique_ptr<llvm::LLVMContext> CodeGen::TheContext;
std::unique_ptr<llvm::IRBuilder<>> CodeGen::Builder;
std::unique_ptr<llvm::Module> CodeGen::TheModule;

void CodeGen::initialize() {
    TheContext = std::make_unique<llvm::LLVMContext>();
    Builder = std::make_unique<llvm::IRBuilder<>>(*TheContext);
    TheModule = std::make_unique<llvm::Module>("Blue Lagoon", *TheContext);
}

llvm::LLVMContext& CodeGen::getContext() {
    return *TheContext;
}

llvm::IRBuilder<>& CodeGen::getBuilder() {
    return *Builder;
}

llvm::Module& CodeGen::getModule() {
    return *TheModule;
}