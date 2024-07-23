use crate::ast::*;
/// Converts a C AST to an assembly AST.
///
/// # Arguments
///
/// * `ast` - The C AST to be converted.
///
/// # Returns
///
/// * `Result<AssemblyProgram, String>` - The assembly AST if conversion is successful, otherwise an error message.
pub fn generate_assembly(ast: Program) -> Result<AsmProgram,String> {
    let mut instructions: Vec<AsmInstruction> = Vec::new();
    if let Statement::Return(exp) = ast.func.body {
        let operand: AsmOperand = generate_operand(exp)?;
        instructions.push(AsmInstruction::Mov(operand, AsmOperand::Register));
        instructions.push(AsmInstruction::Ret);
    } else {
        return Err("Invalid function body, expected a return statement.".to_string());
    }
    Ok(AsmProgram {
        function: AsmFunction {
            name: ast.func.name,
            instructions,
        }})
}

/// A helper function that converts an expression in the C AST to an operand in the assembly AST.
///
/// # Arguments
///
/// * `exp` - The expression to be converted.
///
/// # Returns
///
/// * `Result<Operand, String>` - The operand if conversion is successful, otherwise an error message.
fn generate_operand(exp:Exp) -> Result<AsmOperand, String> {
    match exp {
        Exp::Const(value) => Ok(AsmOperand::Imm(value)),
        _ => Err("Unsupported expression type.".to_string()),
    }
}

/// Converts an assembly AST to a string representation of the assembly code.
///
/// # Arguments
///
/// * `assembly` - The assembly AST to be converted.
///
/// # Returns
///
/// * `String` - The string representation of the assembly code.
pub fn assembly_to_string(assembly: AsmProgram) -> String {
    let mut asm: String = String::new();

    asm.push_str(&format!(" .globl {}\n{}:\n", assembly.function.name, assembly.function.name));
    for instruction in assembly.function.instructions {
        match instruction {
            AsmInstruction::Mov(src, dst) => {
                asm.push_str(&format!("    movl {}, {}\n", operand_to_str(src), operand_to_str(dst)));
            },
            AsmInstruction::Ret => {
                asm.push_str("    ret\n");
            }
        }
    }
    asm.push_str(r#"    .section .note.GNU-stack,"",@progbits"#);
    return asm;
}

/// Converts an operand to its string representation.
///
/// # Arguments
///
/// * `op` - The operand to be converted.
///
/// # Returns
///
/// * `String` - The string representation of the operand.
fn operand_to_str(operand: AsmOperand) -> String {
    match operand {
        AsmOperand::Imm(value) => format!("${}", value),
        AsmOperand::Register => "%eax".to_string(),
    }
}