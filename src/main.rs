// Here we're importing required modules from the Rust standard library and third-party libraries.
// This is akin to ES6 `import` in TypeScript.

// These imports are from the `swc_common` library which provides utility functions for handling 
// JavaScript and TypeScript source code.
use swc_common::BytePos;  // A type to represent a position in a string.

// These imports are from the `swc_ecma_parser` library. It is responsible for parsing JS and TS code.
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax, TsConfig, StringInput};
use swc_ecma_ast::*;  // This library represents the abstract syntax tree (AST) of JS/TS.

#[tokio::main]  // This attribute sets up the Tokio runtime for your main function.
async fn main() {
    // Read content from the file 'sample.ts'. This is similar to the `fs.readFileSync` method in Node.js.
    // let code = std::fs::read_to_string("sample.ts").expect("Failed to read file");
    let code = tokio::fs::read_to_string("sample.ts").await.expect("Failed to read file");
    
    // Configure the parser to handle TypeScript with certain features enabled.
    // Similar to setting up a TypeScript compiler or transpiler with specific options.
    let syntax = Syntax::Typescript(TsConfig {
        tsx: true,  // Indicates if JSX/TSX syntax should be parsed.
        decorators: true,  // Enables parsing of decorators (i.e., `@decorator` in TS).
        dts: true,  // Indicates if it should parse `.d.ts` files.
        no_early_errors: false,  // If set to true, it won't throw errors immediately.
        ..Default::default()  // Fill in the rest of the options with their default values.
    });
    
    // These BytePos values define the range of the source code string we want to parse.
    // Think of them as starting and ending indices in a string.
    let start = BytePos(0);
    let end = BytePos(code.len() as u32);

    // Set up the input for the lexer, defining what part of the `code` string it should tokenize.
    let lexer_input = StringInput::new(&code, start, end);

    // Lexer tokenizes the input source code. Think of it as breaking the code into individual pieces (tokens).
    let lexer = Lexer::new(syntax, EsVersion::Es2020, lexer_input, None);
    
    // Parser takes tokens from the lexer and constructs the AST.
    let mut parser = Parser::new_from(lexer);

    // Parse the tokenized input and construct a module-level AST.
    let module = parser
        .parse_module()
        .expect("Failed to parse TypeScript module");

    // Iterate through the top-level statements in the parsed module.
    for stmt in module.body {
        match stmt {
            // If a statement declares a function, print its name.
            ModuleItem::Stmt(Stmt::Decl(Decl::Fn(fn_decl))) => {
                println!("Found function: {}", fn_decl.ident.sym);
            }
            // For all other types of statements, do nothing.
            _ => {}
        }
    }
}
