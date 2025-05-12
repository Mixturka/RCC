use rcc::{code_gen::{asm_ast::AsmProgram, code_emitter::{amd64::Amd64CodeEmitter, CodeEmitter}}, lexer::Lexer, parser::Parser};

fn main() {
   let mut lexer: Lexer = Default::default();
   println!("{} {}", ' '.is_alphanumeric(), ' '.is_alphabetic());
   let tokens = match lexer.tokenize("main.c") {
    Ok(v) => v,
    Err(e) => panic!()
   };
   println!("{:?}", tokens);

   let mut parser = Parser::default();
   let ast = parser.parse(tokens).unwrap();

   println!("{}", ast);

   let asm_ast: AsmProgram = ast.into();
   println!("{}", asm_ast);

   let mut code_emitter = Amd64CodeEmitter::new("test.s");
   code_emitter.emit_program(&asm_ast);
}
