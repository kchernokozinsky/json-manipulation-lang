use lexer::Lexer;

fn main() {
    let mut lex = Lexer::new("{ a: if () a.5 else \"b\" }");

    for token in lex {
        println!("{:?}", token);
    }
}
