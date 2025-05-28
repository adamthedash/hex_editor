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

    #[token("*")]
    Wildcard,

    #[regex(r"[a-zA-Z]\w*", |x| x.slice().to_string())]
    Identifier(String),

    #[token("_")]
    Placeholder,

    #[token("TAKE_UNTIL")]
    TakeUntil,

    #[token("TAKE_N")]
    TakeN,

    #[token("TAKE_OVER")]
    TakeOver,

    #[regex("(u8|u(16|32|64|128)(be|le)|char)", |x| x.slice().to_string())]
    DType(String),
}
