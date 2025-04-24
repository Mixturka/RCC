use crate::lexer::{Token, TokenType};
use crate::ast::{Expression, FunctionDefinition, Program, Statement};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEof(String),
}

pub struct Parser {
    cur_token: usize
}

impl Default for Parser {
    fn default() -> Self {
        Parser { cur_token: 0 }
    }
}

impl Parser {
    pub fn parse(&mut self, tokens: &Vec<Token>) -> Result<Program, ParseError> {
        Ok(Program::Program(
            self.parse_function(tokens)?
        ))
    }

    fn parse_function(&mut self, tokens: &Vec<Token>) -> Result<FunctionDefinition, ParseError> {
        self.expect("int", tokens)?;
        let func_name_token = tokens.get(self.cur_token).ok_or_else(|| ParseError::UnexpectedEof(
            format!("Unexpected EOF, expected")
        ))?;
        self.cur_token += 1;

        let func_name: &str = match func_name_token.token_type {
            TokenType::Identifier => func_name_token.lexeme.as_ref(),
            _ => return Err(ParseError::UnexpectedToken(
                format!("Unexpected token found: {}, expected function name identifier", &func_name_token.lexeme)
            ))
        };
        self.expect("(", tokens)?; self.expect("void", tokens)?; self.expect(")", tokens)?;
        self.expect("{", tokens)?;
        let sttmt = self.parse_statement(tokens)?;
        self.expect("}", tokens)?;
        
        Ok(FunctionDefinition::Function { 
            name: func_name.to_string(),
            body: sttmt,
        })
    }

    fn parse_statement(&mut self, tokens: &Vec<Token>) -> Result<Statement, ParseError> {
        self.expect("return", tokens)?;
        let return_value = self.parse_expression(tokens)?;
        self.expect(";", tokens)?;
        Ok(Statement::Return(return_value))
    }

    fn parse_expression(&mut self, tokens: &Vec<Token>) -> Result<Expression, ParseError> {
        let token = &tokens[self.cur_token];
        match token.token_type {
            TokenType::Number(num) => {
                self.cur_token += 1;
                Ok(Expression::Constant(num))
            }
            _ => Err(ParseError::UnexpectedToken(format!("Unexpected token found: {}, expected number", &token.lexeme)))
        }
    }

    fn expect(&mut self, expected: &str, tokens: &Vec<Token>) -> Result<(), ParseError> {
        let token = &tokens.get(self.cur_token).ok_or_else(|| ParseError::UnexpectedToken(
            format!("Unexpected EOF, expected '{}'", expected)
        ))?;
        if token.lexeme.as_ref() != expected {
           return Err(ParseError::UnexpectedToken(format!("Unexpected token found: {}, expected: {expected}", token.lexeme.as_ref())));
        }
        self.cur_token += 1;
        Ok(())
    }
}