use std::env;
use std::fs;
use std::io;
use std::io::Read;

mod token;
mod pratt;

const USAGE: &'static str = "
A dummy pratt parser that prints infix math as S-expressions
Usage: pratt [<file>]
";

fn main() {
    let mut args = env::args();
    args.next(); // drop the executable's name
    let mut input = String::new();
    // zero args means use stdin, one arg is a filename.
    match args.next() {
        None => {
            let _ = io::stdin().read_to_string(&mut input);
        }
        Some(filename) => {
            if filename == "--help" {
                return println!("{}", USAGE);
            }
            let _ = fs::File::open(filename.as_str())
                .expect("error opening file")
                .read_to_string(&mut input);
        }
    };

    let tokens = token::lex(input.as_str()).expect("lex error");
    println!("lexed as: {:?}", tokens);

    let mut parser = pratt::sample_parser();
    parser.load_input(tokens);
    let ast = parser.expression(0);
    println!("parses to: {:?}", ast);

    // let result = eval(ast);
    // println!("evaluates to: {:?}", result);

    println!(
        "There are {} unused tokens",
        parser.tokens.len() - parser.index
    );
}

#[allow(dead_code)]
fn eval(_node: pratt::AST) -> i16 {
    panic!("eval not implemented")
}
