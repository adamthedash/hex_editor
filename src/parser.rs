use crate::lexer::Token;
use chumsky::{
    IterParser, Parser,
    prelude::{just, recursive},
    select,
};

#[derive(Clone, Debug)]
pub enum DType {
    U8,
    U16,
    U32,
    U64,
    U128,
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
}

pub fn expr_parser<'a>() -> impl Parser<'a, &'a [Token], Expr> {
    let dtype = select! {
        Token::DType(x) => match x.as_str() {
            "u8" => DType::U8,
            "u16" => DType::U16,
            "u32" => DType::U32,
            "u64" => DType::U64,
            "u128" => DType::U128,
            _ => panic!("Invalid dtype")
        }
    };
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
            expr.repeated()
                .collect()
                .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
                .map(Expr::TakeUntil),
        );

        primative.or(take_until)
    })
}
