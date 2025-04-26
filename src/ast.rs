use std::fmt;

use crate::{code_gen, utils};

type Identifier = String;

pub enum Program {
    Program(FunctionDefinition)
}

pub enum FunctionDefinition {
    Function {
        name: Identifier,
        body: Statement
    }
}

pub enum Statement {
    Return(Expression)
}

pub enum Expression {
    Constant(i32)
}

impl Program {
    fn to_string(&self, indent: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program {{")?;
        match self {
            Self::Program(func) => {
                func.to_string(indent + 1, f)?;
            }
        }
        writeln!(f, "}}")
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string(0, f)
    }
}

impl FunctionDefinition {
    fn to_string(&self, indent: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        utils::write_indent(indent, f)?;
        writeln!(f, "Function {{")?;
        match self {
            Self::Function{name, body} => {
                utils::write_indent(indent + 1, f)?;
                writeln!(f, "name: {name}")?;
                utils::write_indent(indent + 1, f)?;
                writeln!(f, "body {{")?;
                body.to_string(indent + 2, f)?;
                utils::write_indent(indent + 1, f)?;
                writeln!(f, "}}")?;
            }
        }
        utils::write_indent(indent, f)?;
        writeln!(f, "}}")
    }
}

impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string(0, f)
    }
}

impl Statement {
    fn to_string(&self, indent: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        utils::write_indent(indent, f)?;
        writeln!(f, "Statement {{")?;
        match self {
            Self::Return(exp) => {
                utils::write_indent(indent + 1, f)?;
                writeln!(f, "Return {{")?;
                exp.to_string(indent + 2, f)?;
                utils::write_indent(indent + 1, f)?;
                writeln!(f, "}}")?;
            }
        }
        utils::write_indent(indent, f)?;
        writeln!(f, "}}")
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string(0, f)
    }
}

impl Expression {
    fn to_string(&self, indent: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        utils::write_indent(indent, f)?;
        writeln!(f, "Expression {{")?;
        match self {
            Self::Constant(num) => {
                utils::write_indent(indent + 1, f)?;
                writeln!(f, "Constant({num})")?;
            }
        }
        utils::write_indent(indent, f)?;
        writeln!(f, "}}")
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string(0, f)
    }
}

impl TryFrom<code_gen::asm_ast::AsmProgram> for Program {
    type Error = code_gen::CodegenError;

    fn try_from(value: code_gen::asm_ast::AsmProgram) -> Result<Self, Self::Error> {
        
    }
}