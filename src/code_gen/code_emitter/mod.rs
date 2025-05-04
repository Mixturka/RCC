pub mod amd64;

use super::asm_ast::{AsmFunctionDefinition, AsmProgram, Instruction, Operand};

pub enum CodeEmitterError {

}

pub trait CodeEmitter {
    fn emit_program(&mut self, prog: &AsmProgram);
    fn emit_function(&mut self, func: &AsmFunctionDefinition);
    fn emit_instructions(&mut self, instructions: &[Instruction]);
    fn emit_operand(&mut self, operand: &Operand);
}