#[cfg(test)]
mod tests;

mod display;
mod tokenizer;
use span::Span;

pub use display::*;
pub use tokenizer::*;

/// TokenKind represent the kind of token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// A token representing a keyword, e.g. `let`.
    Keyword(KeywordKind),
    /// A token representing a literal, e.g. `42`.
    Literal(LiteralKind),
    /// A token representing an identifier, e.g. `foo`.
    Identifier,
    /// A token representing a symbol, e.g. `+`.
    Symbol(SymbolKind),
    /// A token representing a comment, e.g. `// this is a comment`.
    Comment,
    /// A token representing a whitespace, e.g. ` `.
    Whitespace,
    /// An unknown token.
    Unknown,
    /// End of file.
    Eof,
}

/// Keyword represents a keyword token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeywordKind {
    /// `let` keyword.
    Let,
    /// `fn` keyword.
    Fn,
    /// `if` keyword.
    If,
    /// `else` keyword.
    Else,
    /// `return` keyword.
    Return,
    /// `true` keyword.
    True,
    /// `false` keyword.
    False,
    /// `pub` keyword.
    Pub,
    /// `import` keyword.
    Import,
    /// `as` keyword.
    As,
    /// `extern` keyword.
    Extern,
    /// `const` keyword.
    Const,
    /// `mut` keyword.
    Mut,
}

/// Literal represents a literal token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    /// A string literal, e.g. `"foo"`.
    String,
    /// An integer literal, e.g. `42`.
    Integer,
}

/// Symbol represents a symbol token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    /// `+` symbol.
    Plus,
    /// `-` symbol.
    Minus,
    /// `*` symbol.
    Asterisk,
    /// `/` symbol.
    Slash,
    /// `!` symbol.
    Bang,
    /// `=` symbol.
    Equal,
    /// `==` symbol.
    EqualEqual,
    /// `!=` symbol.
    BangEqual,
    /// `<` symbol.
    Less,
    /// `>` symbol.
    Greater,
    /// `<=` symbol.
    LessEqual,
    /// `>=` symbol.
    GreaterEqual,
    /// `<<` symbol.
    LeftShift,
    /// `>>` symbol.
    RightShift,
    /// `(` symbol.
    LeftParen,
    /// `)` symbol.
    RightParen,
    /// `{` symbol.
    LeftBrace,
    /// `}` symbol.
    RightBrace,
    /// `[` symbol.
    LeftBracket,
    /// `]` symbol.
    RightBracket,
    /// `,` symbol.
    Comma,
    /// `.` symbol.
    Dot,
    /// `..` symbol.
    DotDot,
    /// `:` symbol.
    Colon,
    /// `;` symbol
    Semi,
    /// `%` symbol.
    Percent,
    /// `^` symbol.
    Caret,
    /// `&` symbol.
    And,
    /// `&&` symbol.
    AndAnd,
    /// `|` symbol.
    Or,
    /// `||` symbol.
    OrOr,
    /// `~` symbol.
    Tilde,
    /// `->` symbol.
    Arrow,
}

/// Token represents a single token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    /// The kind of the token.
    pub kind: TokenKind,
    /// The span of the token.
    pub span: Span,
}

impl Token {
    /// Creates a new `Token`.
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
