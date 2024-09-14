// src/main.cpp
#include "lexer.h"
#include "parser.h"
#include "codegen.h"
#include <iostream>
#include <llvm/Support/raw_ostream.h>

int main() {
    CodeGen::initialize();

    std::string input = "3 + 4 - 2";
    Lexer lexer(input);
    Parser parser(lexer);

    try {
        auto ast = parser.parse();
        llvm::Value* result = ast->codegen();

        // Print the generated LLVM IR
        CodeGen::getModule().print(llvm::outs(), nullptr);

        std::cout << "Compilation successful!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}