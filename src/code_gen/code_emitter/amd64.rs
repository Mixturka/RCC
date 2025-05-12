use std::{fs::File, io::{BufWriter, Write}};

use super::{CodeEmitter, EmitResult};

use crate::code_gen::asm_ast::{AsmFunctionDefinition, AsmProgram, Instruction, Operand};

pub struct Amd64CodeEmitter {
    file_writer: BufWriter<File>
}

impl Amd64CodeEmitter {
    pub fn new(file_path: &str) -> Amd64CodeEmitter {
        let file = File::create(file_path).expect(format!("Error opening file {}", file_path).as_str());
        let file_writer = BufWriter::new(file);
        Amd64CodeEmitter { file_writer }
    }
}

impl CodeEmitter for Amd64CodeEmitter {
    fn emit_program(&mut self, prog: &AsmProgram) -> EmitResult {
        match prog {
            AsmProgram::Program(func) => self.emit_function(func)?,
        }

        #[cfg(target_os = "linux")]
        {
            self.file_writer.write(br#"  .section .note.GNU-stack,"",@progbits"#)?;
            self.file_writer.write(b"\n")?;
        }

        Ok(())
    }

    fn emit_function(&mut self, func: &AsmFunctionDefinition) -> EmitResult {
        match func {
            AsmFunctionDefinition::Function { name, instructions } => {
                write!(self.file_writer, "  .globl {}\n", name)?;
                write!(self.file_writer, "{}:\n", name)?;
                self.emit_instructions(instructions)?;
            }
        }
        Ok(())
    }

    fn emit_instructions(&mut self, instructions: &[Instruction]) -> EmitResult {
         for instruction in instructions {
            match instruction {
                Instruction::Mov { src, dst } => {
                    self.file_writer.write(b"  movl ")?;
                    self.emit_operand(&src)?;
                    self.file_writer.write(b", ")?;
                    self.emit_operand(&dst)?;
                    self.file_writer.write(b"\n")?;
                },
                Instruction::Ret => {
                    self.file_writer.write(b"  ret\n")?;
                }
            }
        }

        Ok(())
    }

    fn emit_operand(&mut self, operand: &Operand) -> EmitResult {
        match operand {
            Operand::Imm(val) => {
                write!(self.file_writer, "${}", val)?;
            }
            Operand::Register(reg) => {
                write!(self.file_writer, "%{}", reg)?;
            }
        }
        Ok(())
    }
}