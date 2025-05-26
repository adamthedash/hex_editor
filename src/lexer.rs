use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // whitespace
pub enum Token {
    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[regex(r"\d+", |x| x.slice().parse::<u64>().expect("Failed to parse number"))]
    Number(u64),

    #[regex(r"[a-zA-Z]\w*", |x| x.slice().to_string())]
    Identifier(String),

    #[token("_")]
    Placeholder,

    #[token("TAKE_UNTIL")]
    TakeUntil,

    #[token("TAKE_N")]
    TakeN,

    #[regex("(u(8|16|32|64|128)|char)", |x| x.slice().to_string())]
    DType(String),
}
