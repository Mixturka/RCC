use crate::ast;

type Identifier = String;

pub enum AsmProgram {
    Program(FunctionDefinition)
}

pub enum FunctionDefinition {
    Function {
        name: Identifier,
        instructions: Box<[Instruction]>
    }
}

pub enum Instruction {
    Mov {
        src: Operand,
        dst: Operand
    },
    Ret
}

pub enum Operand {
    Imm(i32), // Immediate value
    Register(Box<str>)
}
