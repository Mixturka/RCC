use rcc::lexer::Lexer;

fn main() {
   let mut lexer: Lexer = Default::default();
   println!("{} {}", ' '.is_alphanumeric(), ' '.is_alphabetic());
   let tokens = match lexer.tokenize("/home/mixturka/Documents/Rust/rcc/rcc/main.c") {
    Ok(v) => v,
    Err(e) => panic!()
   };
   println!("{:?}", tokens);
}
