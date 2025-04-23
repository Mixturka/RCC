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