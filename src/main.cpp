#include "lexer.h"
#include "parser.h"
#include "codegen.h"

int main() {
    // Initialize the module for code generation.
    TheModule = std::make_unique<llvm::Module>("Blue Lagoon JIT", TheContext);

    // Prime the first token.
    getNextToken();

    // Parse and generate IR.
    while (true) {
        switch (CurTok) {
        case tok_eof:
            return 0;
        case tok_def:
            // Handle function definitions (to be added later).
            break;
        case ';': // Skip to the next token if a semicolon is found.
            getNextToken();
            break;
        default:
            // Handle top-level expressions.
            if (auto Expr = ParseExpression()) {
                llvm::Value *IR = Expr->codegen();
                if (IR)
                    IR->print(llvm::errs());
            }
            break;
        }
    }

    // Print the IR code.
    TheModule->print(llvm::errs(), nullptr);
    return 0;
}
