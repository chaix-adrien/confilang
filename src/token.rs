// mod token;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Basic
    Illegal(String),
    Eof,

    // Identifiers and literals
    Ident(String),
    Int(i32),
    Float(f32),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Let,
    Fn,
}
