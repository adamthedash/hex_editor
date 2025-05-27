use crate::lexer::Token;
use chumsky::{
    IterParser, Parser,
    error::Rich,
    extra,
    prelude::{just, recursive},
    select,
};

#[derive(Clone, Debug)]
pub enum Endianness {
    Big,
    Little,
}

#[derive(Clone, Debug)]
pub enum DType {
    U8,
    U16(Endianness),
    U32(Endianness),
    U64(Endianness),
    U128(Endianness),
    Char,
}

#[derive(Debug, Clone)]
pub enum Count {
    Number(u64),
    Identifier(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Primative {
        dtype: DType,
        count: Count,
        identifier: Option<String>,
    },
    TakeUntil(Vec<Expr>),
    TakeN {
        count: Count,
        exprs: Vec<Expr>,
    },
}

pub fn expr_parser<'a>() -> impl Parser<'a, &'a [Token], Expr, extra::Err<Rich<'a, Token>>> {
    let dtype = select! {
        Token::DType(x) => match x.as_str() {
            "u8" => DType::U8,
            "u16le" => DType::U16(Endianness::Little),
            "u32le" => DType::U32(Endianness::Little),
            "u64le" => DType::U64(Endianness::Little),
            "u128le" => DType::U128(Endianness::Little),
            "u16be" => DType::U16(Endianness::Big),
            "u32be" => DType::U32(Endianness::Big),
            "u64be" => DType::U64(Endianness::Big),
            "u128be" => DType::U128(Endianness::Big),
            "char" => DType::Char,
            _ => panic!("Invalid dtype")
        }
    }
    .labelled("dtype");
    let count = select! {
        Token::Number(n) => Count::Number(n),
        Token::Identifier(id) => Count::Identifier(id)
    };

    let identifier = select! {
        Token::Identifier(id) => Some(id),
        Token::Placeholder => None
    };

    let primative = dtype
        .then(count)
        .then(identifier)
        .map(|((dtype, count), identifier)| Expr::Primative {
            dtype,
            count,
            identifier,
        });

    recursive(|expr| {
        let take_until = just(Token::TakeUntil).ignore_then(
            expr.clone()
                .repeated()
                .collect()
                .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
                .map(Expr::TakeUntil),
        );

        let take_n = just(Token::TakeN)
            .ignore_then(count)
            .then(
                expr.repeated()
                    .collect()
                    .delimited_by(just(Token::LeftBrace), just(Token::RightBrace)),
            )
            .map(|(count, exprs)| Expr::TakeN { count, exprs });

        primative.or(take_until).or(take_n)
    })
}
