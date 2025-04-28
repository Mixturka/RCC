use rcc::{code_gen::asm_ast::AsmProgram, lexer::Lexer, parser::Parser};

fn main() {
   let mut lexer: Lexer = Default::default();
   println!("{} {}", ' '.is_alphanumeric(), ' '.is_alphabetic());
   let tokens = match lexer.tokenize("/home/mixturka/Documents/Rust/rcc/rcc/main.c") {
    Ok(v) => v,
    Err(e) => panic!()
   };
   println!("{:?}", tokens);

   let mut parser = Parser::default();
   let ast = parser.parse(tokens).unwrap();

   println!("{}", ast);

   let asm_ast: AsmProgram = ast.into();
   println!("{}", asm_ast);
}
