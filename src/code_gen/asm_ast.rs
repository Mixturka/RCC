use std::fmt;
use std::string::ToString;

use crate::{ast, utils};

type Identifier = Box<str>;

pub enum AsmProgram {
    Program(AsmFunctionDefinition)
}

pub enum AsmFunctionDefinition {
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

impl From<ast::Program> for AsmProgram {
    fn from(value: ast::Program) -> Self {
        convert_program(&value)
    }
}

fn convert_program(prog: &ast::Program) -> AsmProgram {
    match prog {
        ast::Program::Program(func) => {
            let func_def = convert_function(func);
            AsmProgram::Program(func_def)
        }
    } 
}

fn convert_function(func_def: &ast::FunctionDefinition) -> AsmFunctionDefinition {
    match func_def {
        ast::FunctionDefinition::Function { name, body } => {
            let asm_instructions = convert_statement(body);
            AsmFunctionDefinition::Function {
                name: name.to_string().into_boxed_str(),
                instructions: asm_instructions.into_boxed_slice(),
            }
        }
    }
}

fn convert_statement(stmt: &ast::Statement) -> Vec<Instruction> {
    match stmt {
        ast::Statement::Return(exp) => {
            let mut instructions: Vec<Instruction> = Vec::new();
            let return_val = convert_expression(exp);
            instructions.push(Instruction::Mov { src: return_val, dst: Operand::Register("EAX".into())});
            instructions.push(Instruction::Ret);
            instructions
        }
    }
}

fn convert_expression(exp: &ast::Expression) -> Operand {
    match exp {
        ast::Expression::Constant(val) => Operand::Imm(*val)
    }
}

impl AsmProgram {
    fn to_string(&self, indent: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program {{")?;
        match self {
            Self::Program(func) => {
                func.to_string(indent + 1, f)?;
            }
        }
        write!(f, "}}")
    }
}

impl fmt::Display for AsmProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string(0, f)
    }
}

impl AsmFunctionDefinition {
    fn to_string(&self, indent: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        utils::write_indent(indent, f)?;
        writeln!(f, "Function {{")?;
        match self {
            Self::Function{name, instructions} => {
                utils::write_indent(indent + 1, f)?;
                writeln!(f, "name: {name}")?;
                utils::write_indent(indent + 1, f)?;
                writeln!(f, "instructions {{")?;
                // instructions.to_string(indent + 2, f)?;
                for i in instructions {
                    // utils::write_indent(indent + 2, f)?;
                    i.to_string(indent + 2, f)?;
                }

                utils::write_indent(indent + 1, f)?;
                writeln!(f, "}}")?;
            }
        }
        utils::write_indent(indent, f)?;
        writeln!(f, "}}")
    }
}

impl fmt::Display for AsmFunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string(0, f)
    }
}

impl Instruction {
    fn to_string(&self, indent: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        utils::write_indent(indent, f)?;
        match self {
            Self::Mov { src, dst } => {
                writeln!(f, "mov {}, {}", src.to_string(), dst.to_string())
            }
            Self::Ret => { writeln!(f, "ret") }
        }
        // utils::write_indent(indent, f)?;
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string(0, f)
    }
}

impl ToString for  Operand {
    fn to_string(&self) -> String {
        match self {
            Self::Imm(val) => format!("Imm({})", val),
            Self::Register(r) => format!("Register({})", r)
        }
    }    
}