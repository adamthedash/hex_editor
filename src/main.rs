use std::fs;

use chumsky::{IterParser, Parser};
use display::print_data;
use interpreter::process_bytes;
use logos::Logos;

mod display;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let file = fs::read_to_string("./data/patterns/png.pattern").unwrap();
    // let file = fs::read_to_string("./data/patterns/empty_take_until.pattern").unwrap();
    // let file = fs::read_to_string("./data/patterns/nested_take_untils.pattern").unwrap();
    // let file = fs::read_to_string("./data/patterns/shallow_mixed.pattern").unwrap();
    // let file = fs::read_to_string("./data/patterns/many_primative.pattern").unwrap();

    let lex = lexer::Token::lexer(&file);
    let tokens = lex.collect::<Result<Vec<_>, _>>().unwrap();
    println!("{:?}", tokens);

    let parser = parser::expr_parser().repeated().collect::<Vec<_>>();

    let pattern = parser
        .parse(&tokens)
        .into_result()
        .expect("Failed to parse");
    println!("{:#?}", pattern);

    let png_bytes =
        fs::read("/home/adam/Pictures/Screenshots/Screenshot from 2025-05-24 22-53-30.png")
            .unwrap();

    let mut png_iter = png_bytes.into_iter().peekable();

    let parsed = process_bytes(&pattern, &mut png_iter);
    println!("{:?}", parsed);

    print_data(&parsed, &[]);
}
