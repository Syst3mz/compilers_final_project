use crate::parser::token_kind::TokenKind;

type T = TokenKind;
pub const LEX_TABLE : [(&'static str, TokenKind); 28]= [
    ("while", T::While),
    ("true", T::True),
    ("false", T::False),
    ("bool", T::BoolType),
    ("list", T::ListType),
    ("else", T::Else),
    ("int", T::IntType),
    ("let", T::Let),
    ("fn", T::Fn),
    ("if", T::If),
    ("==", T::EqualsEquals),
    ("||", T::PipePie),
    ("&&", T::AndAnd),
    ("->", T::Arrow),
    ("{", T::LCurlyBrace),
    ("}", T::RCurlyBrace),
    (";", T::Semicolon),
    ("=", T::Equals),
    (":", T::Colon),
    ("(", T::LParen),
    (")", T::RParen),
    ("+", T::Plus),
    (">", T::RAngleBracket),
    ("-", T::Minus),
    ("[", T::LBracket),
    ("]", T::RBracket),
    (",", T::Comma),
    ("!", T::Bang)
];