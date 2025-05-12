pub mod amd64;

use super::asm_ast::{AsmFunctionDefinition, AsmProgram, Instruction, Operand};

pub enum CodeEmitterError {
    Io(std::io::Error),
}

impl From<std::io::Error> for CodeEmitterError {
    fn from(err: std::io::Error) -> Self {
        CodeEmitterError::Io(err)
    }
}

type EmitResult = Result<(), CodeEmitterError>;

pub trait CodeEmitter {
    fn emit_program(&mut self, prog: &AsmProgram) -> EmitResult;
    fn emit_function(&mut self, func: &AsmFunctionDefinition) -> EmitResult;
    fn emit_instructions(&mut self, instructions: &[Instruction]) -> EmitResult;
    fn emit_operand(&mut self, operand: &Operand) -> EmitResult;
}