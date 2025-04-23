use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum TokenType {
    Identifier,
    Number(i64),
    Int,
    Void,
    Return,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Eof
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: Box<str>,
}

impl Token {
    fn from_boxed_str(token_type: TokenType, lexeme: Box<str>) -> Self {
        Self {
            token_type,
            lexeme
        }
    }

    fn from_char(token_type: TokenType, lexeme: char) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string().into_boxed_str()
        }
    }
}

#[derive(Debug)]
pub enum LexError {
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
    Lex{message: String}
}

impl From<std::io::Error> for LexError {
    fn from(err: std::io::Error) -> Self {
        LexError::Io(err)
    }
}

impl From<std::str::Utf8Error> for LexError {
    fn from(err: std::str::Utf8Error) -> Self {
        LexError::Utf8(err)
    }
}

enum LexerState {
    Number,
    Identifier,
    Other
}

pub struct Lexer {
    state: LexerState,
    tokens: Vec<Token>,
    line: usize,
    column: usize
}

impl Default for Lexer {
    fn default() -> Self {
        Lexer { state: LexerState::Other,
            line: 0,
            column: 0,
            tokens: Vec::new(),
        }
    }
}

impl Lexer {
    pub fn tokenize(&mut self, filepath: &str) -> Result<&Vec<Token>, LexError> {
        let f = File::open(filepath)?;
        let mut reader = BufReader::new(f);
        let mut lexeme = String::new();
        let mut consumed_bytes: usize = 0;
        
        loop {
            let buffer = reader.fill_buf()?;
            if buffer.is_empty() {
                self.handle_eof(&mut lexeme);
                self.tokens.push(Token::from_char(TokenType::Eof, '\0'));
                break;
            }

            let buf_str = std::str::from_utf8(buffer)?;
            consumed_bytes += match self.state {
                LexerState::Identifier => self.read_identifier(buf_str, &mut lexeme),
                LexerState::Number => self.read_number(buf_str, &mut lexeme),
                LexerState::Other => self.read_other(buf_str, &mut lexeme),
            };
            
            reader.consume(consumed_bytes);
            consumed_bytes = 0;
        }
        Ok(&self.tokens)
    }

    fn read_identifier(&mut self, buf_str: &str, lexeme: &mut String) -> usize {
        let mut consumed_bytes: usize = 0;
        
        for (byte, ch) in buf_str.char_indices() {
            if ch.is_alphabetic() || ch == '_' || (!lexeme.starts_with(|c: char| c.is_numeric()) && ch.is_alphanumeric()) {
                lexeme.push(ch);
            } else {
                let token_type = self.check_keyword(lexeme);
                self.tokens.push(Token::from_boxed_str(token_type, lexeme.clone().into_boxed_str()));
                lexeme.clear();
                consumed_bytes = byte;
                self.state = LexerState::Other;
                break;
            }
            consumed_bytes = byte + ch.len_utf8();
        }
        consumed_bytes
    }

    fn read_other(&mut self, buf_str: &str, lexeme: &mut String) -> usize {
        let mut consumed_bytes = 0;

        if let Some(ch) = buf_str.chars().next() {
            consumed_bytes = ch.len_utf8();
            match ch {
                '(' => self.tokens.push(Token::from_char(TokenType::LeftParen, ch)),
                ')' => self.tokens.push(Token::from_char(TokenType::RightParen, ch)),
                '{' => self.tokens.push(Token::from_char(TokenType::LeftBrace, ch)),
                '}' => self.tokens.push(Token::from_char(TokenType::RightBrace, ch)),
                ';' => self.tokens.push(Token::from_char(TokenType::Semicolon, ch)),
                'a'..='z' | 'A'..='Z' => {
                    self.state = LexerState::Identifier;
                    lexeme.push(ch);
                },
                '0'..='9' => {
                    self.state = LexerState::Number;
                    lexeme.push(ch);
                },
                ' ' | '\r' | '\t' | '\n' => {},
                _ => {}
            }
        }
        consumed_bytes
    }

    fn read_number(&mut self, buf_str: &str, lexeme: &mut String) -> usize {
        let mut consumed_bytes: usize = 0;
        for (byte, ch) in buf_str.char_indices() {
            if ch.is_numeric() {
                lexeme.push(ch);
            } else {
                self.tokens.push(Token::from_boxed_str(TokenType::Number(lexeme.parse::<i64>().unwrap()), lexeme.clone().into_boxed_str()));
                lexeme.clear();
                self.state = LexerState::Other;
                consumed_bytes = byte;
                break;
            }
            consumed_bytes = byte + ch.len_utf8();
        }

        consumed_bytes
    }

    fn handle_eof(&mut self, lexeme: &mut String) {
        match self.state {
            LexerState::Identifier => {
                let token_type = self.check_keyword(lexeme);
                self.tokens.push(Token::from_boxed_str(token_type, lexeme.clone().into_boxed_str()));
                lexeme.clear();
            }
            LexerState::Number => self.tokens.push(Token::from_boxed_str(TokenType::Number(lexeme.parse::<i64>().unwrap()), lexeme.clone().into_boxed_str())),
            LexerState::Other => ()
        }
    }

    fn check_keyword(&self, lexeme: &str) -> TokenType {
        match lexeme {
            "void" => TokenType::Void,
            "int" => TokenType::Int,
            "return" => TokenType::Return,
            _ => TokenType::Identifier
        }
    }
}