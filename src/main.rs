mod ast;
mod lex;
mod parse;
mod assembly;

use std::fs::File;
use std::io::stdin;
use std::process::Command;
use crate::{
    lex::lex, 
    parse::parse, 
    assembly::{generate_assembly,assembly_to_string},
    ast::*,
};
fn main() {
     // Read the file name from standard input
     let mut input = String::new();
     stdin().read_line(&mut input).expect("Failed to read input");
     let input = input.trim(); // trim the input to remove any extraneous whitespace or newlines
 
     // Open the file and lex its contents
     let file: File = File::open(input).expect("Failed to open file");
     let tokens: Vec<Token> = lex(file);
     // Parse the tokens into an AST
     match parse(tokens) {
        Ok(ast) => {
            // Generate assembly from the AST
            match generate_assembly(ast) {
                Ok(assembly_ast) => {
                    // Convert the assembly AST to assembly code
                    let assembly_code = assembly_to_string(assembly_ast);

                    // Write the assembly to a file
                    let assembly_file = "assembly.s";
                    if let Err(e) = std::fs::write(assembly_file, &assembly_code) {
                        eprintln!("Failed to write assembly to file: {}", e);
                        return;
                    }

                    // Assemble the file into an object file
                    let output = Command::new("gcc")
                        .args(&[assembly_file, "-o", "out"])
                        .output()
                        .expect("Failed to execute assembler");

                    if !output.status.success() {
                        eprintln!("Assembler error: {}", String::from_utf8_lossy(&output.stderr));
                        return;
                    }

                    // Clean up intermediate files
                    if let Err(e) = std::fs::remove_file(assembly_file) {
                        eprintln!("Failed to delete assembly file: {}", e);
                    }

                    println!("Executable created successfully.");
                }
                Err(e) => {
                    eprintln!("Assembly generation error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}

