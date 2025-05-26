mod display;
mod interpreter;
mod lexer;
mod parser;

fn main() {}

#[cfg(test)]
mod tests {
    use std::fs;

    use chumsky::{IterParser, Parser};
    use logos::Logos;

    use crate::{display::print_data, interpreter::process_bytes, lexer, parser};

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

        let parsed = process_bytes(&pattern, &mut png_iter);
        println!("{:?}", parsed);

        print_data(&parsed, &[]);
    }

    #[test]
    fn test_poe_bundle_index() {
        let file = fs::read_to_string("./data/patterns/poe_bundle_index.pattern").unwrap();

        let lex = lexer::Token::lexer(&file);
        let tokens = lex.collect::<Result<Vec<_>, _>>().unwrap();
        println!("{:?}", tokens);

        let parser = parser::expr_parser().repeated().collect::<Vec<_>>();

        let pattern = parser
            .parse(&tokens)
            .into_result()
            .expect("Failed to parse");
        println!("{:#?}", pattern);

        let png_bytes = fs::read("/mnt/nvme_4tb/programming/data/poe/cache/patch-poe2.poecdn.com/4.2.0.13/Bundles2/_.index.bin").unwrap();

        let mut png_iter = png_bytes.into_iter().peekable();

        let parsed = process_bytes(&pattern, &mut png_iter);
        println!("{:?}", parsed);

        print_data(&parsed, &[]);
    }
}
