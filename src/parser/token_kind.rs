#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum TokenKind {
    LCurlyBrace, RCurlyBrace,
    Semicolon,
    Equals,
    While,
    Let,
    Colon,
    Fn,
    LParen, RParen,
    Arrow,
    If, Else,

    // exprs and the like
    Plus, EqualsEquals, RAngleBracket, Minus, PipePie, AndAnd, Bang,

    // lists
    LBracket, RBracket, Comma,

    // type stuff
    IntType,
    BoolType,
    ListType,

    Name, Int,
    True, False
}