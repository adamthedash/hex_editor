use std::fs;

use chumsky::{IterParser, Parser as _};
use clap::{Parser, command};
use display::{HexWriter, print_horizontal, print_vertical};
use interpreter::{Stack, process_bytes};
use logos::Logos;

mod display;
mod interpreter;
mod lexer;
mod parser;

#[derive(Parser)]
#[command(name = "pattern-parser")]
#[command(about = "A pattern parser for binary files")]
struct Args {
    /// Path to the pattern file
    pattern_file: String,

    /// Path to the binary file to parse
    binary_file: String,
}

fn main() {
    let args = Args::parse();
    let file = fs::read_to_string(&args.pattern_file).unwrap();

    let tokens = lexer::Token::lexer(&file)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse tokens.");

    let parser = parser::expr_parser().repeated().collect::<Vec<_>>();

    let pattern = parser
        .parse(&tokens)
        .into_result()
        .expect("Failed to parse pattern file.");
    println!("{:#?}", pattern);

    let png_bytes = fs::read(&args.binary_file).unwrap();

    let mut png_iter = png_bytes.into_iter().peekable();

    let mut stack = Stack::new();
    let parsed =
        process_bytes(&pattern, &mut png_iter, &mut stack).expect("Faild to apply pattern");

    //print_vertical(&parsed, &[]);
    let mut writer = HexWriter::new(130);
    print_horizontal(&parsed, &mut writer, &[]);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use chumsky::{IterParser, Parser};
    use logos::Logos;

    use crate::{
        display::print_vertical,
        interpreter::{Stack, process_bytes},
        lexer, parser,
    };

    #[test]
    fn test_png() {
        let file = fs::read_to_string("./data/patterns/png.pattern").unwrap();

        let lex = lexer::Token::lexer(&file);
        let tokens = lex.collect::<Result<Vec<_>, _>>().unwrap();
        println!("{:?}", tokens);

        let parser = parser::expr_parser().repeated().collect::<Vec<_>>();

        let pattern = parser
            .parse(&tokens)
            .into_result()
            .expect("Failed to parse");
        println!("{:#?}", pattern);

        let png_bytes = fs::read("./data/binary_files/image.png").unwrap();

        let mut png_iter = png_bytes.into_iter().peekable();

        let mut stack = Stack::new();
        let parsed =
            process_bytes(&pattern, &mut png_iter, &mut stack).expect("Faild to apply pattern");
        println!("{:?}", parsed);

        print_vertical(&parsed, &[]);
    }

    #[test]
    fn test_poe_bundle_index() {
        let file = fs::read_to_string("./data/patterns/poe_bundle.pattern").unwrap();

        let lex = lexer::Token::lexer(&file);
        let tokens = lex.collect::<Result<Vec<_>, _>>().unwrap();
        println!("{:?}", tokens);

        let parser = parser::expr_parser().repeated().collect::<Vec<_>>();

        let pattern = parser
            .parse(&tokens)
            .into_result()
            .expect("Failed to parse pattern");
        println!("{:#?}", pattern);

        let png_bytes = fs::read("/mnt/nvme_4tb/programming/data/poe/cache/patch-poe2.poecdn.com/4.2.0.13/Bundles2/_.index.bin").unwrap();

        let mut png_iter = png_bytes.into_iter().peekable();

        let mut stack = Stack::new();
        let parsed =
            process_bytes(&pattern, &mut png_iter, &mut stack).expect("Faild to apply pattern");
        println!("{:?}", parsed);

        print_vertical(&parsed, &[]);
        panic!()
    }
}
