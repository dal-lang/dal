use crate::span::Span;

/// Token holds the kind of token and the span of the token in the source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    /// Creates a new token.
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

/// TokenKind is the kind of token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Colon,
    Question,
    Percent,
    Caret,
    At,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    ShiftRight,
    Less,
    LessEqual,
    ShiftLeft,
    Arrow,
    Ellipsis,

    // Literals
    Identifier(String),
    String(String),
    Number(String),

    // Keywords
    Module,
    Import,
    Extern,
    Pub,
    Fn,
    Let,
    Const,
    Mut,
    If,
    Else,
    Return,
    True,
    False,
    As,

    Unknown,

    // EOF
    Eof,
}
