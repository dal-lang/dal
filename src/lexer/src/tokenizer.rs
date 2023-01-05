use std::str::Chars;

use crate::Keyword::*;
use crate::Span;
use crate::Symbol::*;
use crate::LiteralKind;
use crate::Token;
use crate::TokenKind;
use crate::TokenKind::*;

/// Cursor is a wrapper around a `&str` that keeps track of the current position.
///
/// It is peekable, so it can peek at the next character without advancing the cursor.
pub struct Cursor<'a> {
    chars: Chars<'a>,
    pos: usize,
    line: usize,
    span: Span,
}

pub(crate) const EOF_CHAR: char = '\0';

/// Check if the given char considered as a whitespace.
pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

/// Check if the given char is valid as a first character of an identifier.
pub fn is_id_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

/// Check if the given char is valid as a non-first character of an identifier.
pub fn is_id_continue(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_continue(c)
}

/// The parsed string is lexically an identifier.
pub fn is_ident(s: &str) -> bool {
    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        is_id_start(c) && chars.all(is_id_continue)
    } else {
        false
    }
}

/// Creates an iterator that produces tokens from the given source code.
pub fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(source);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != Eof { Some(token) } else { None }
    })
}

impl<'a> Cursor<'a> {
    /// Creates a new `Cursor`.
    pub fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars(),
            pos: 0,
            line: 1,
            span: Span::new(1, 0, 1, 0),
        }
    }

    /// Peek at the next character without advancing the cursor.
    /// Returns `EOF_CHAR` if the cursor is at the end of the string.
    pub(crate) fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Peek at the second character without advancing the cursor.
    /// Returns `EOF_CHAR` if the cursor is at the end of the string.
    pub(crate) fn second(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
    }

    /// Check if the cursor is at the end of the string.
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Move to the next character.
    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
        }
        Some(c)
    }

    /// Eat while the predicate is true or until the end of file is reached.
    pub(crate) fn eat_while<F>(&mut self, mut pred: F)
        where
            F: FnMut(char) -> bool,
    {
        while !self.is_eof() && pred(self.first()) {
            self.bump();
        }
    }

    /// Advance to the next token.
    pub fn advance_token(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token::new(Eof, self.span),
        };
        let token_kand = match first_char {
            // Slash or Comment.
            '/' => match self.first() {
                '/' => self.eat_comment(),
                _ => Symbol(Slash),
            }

            // Multi-symbol operators.
            // = or ==.
            '=' => match self.first() {
                '=' => {
                    self.bump();
                    Symbol(EqualEqual)
                }
                _ => Symbol(Equal),
            }
            // ! or !=.
            '!' => match self.first() {
                '=' => {
                    self.bump();
                    Symbol(BangEqual)
                }
                _ => Symbol(Bang),
            }
            // < or <=.
            '<' => match self.first() {
                '=' => {
                    self.bump();
                    Symbol(LessEqual)
                }
                _ => Symbol(Less),
            }
            // > or >=.
            '>' => match self.first() {
                '=' => {
                    self.bump();
                    Symbol(GreaterEqual)
                }
                _ => Symbol(Greater),
            }
            // & or &&.
            '&' => match self.first() {
                '&' => {
                    self.bump();
                    Symbol(AndAnd)
                }
                _ => Symbol(And),
            }
            // | or ||.
            '|' => match self.first() {
                '|' => {
                    self.bump();
                    Symbol(OrOr)
                }
                _ => Symbol(Or),
            }
            // - or ->.
            '-' => match self.first() {
                '>' => {
                    self.bump();
                    Symbol(Arrow)
                }
                _ => Symbol(Minus),
            }
            // . or ..
            '.' => match self.first() {
                '.' => {
                    self.bump();
                    Symbol(DotDot)
                }
                _ => Symbol(Dot),
            }

            // Single-symbol operators.
            '+' => Symbol(Plus),
            '*' => Symbol(Asterisk),
            '(' => Symbol(LeftParen),
            ')' => Symbol(RightParen),
            '{' => Symbol(LeftBrace),
            '}' => Symbol(RightBrace),
            '[' => Symbol(LeftBracket),
            ']' => Symbol(RightBracket),
            ',' => Symbol(Comma),
            ':' => Symbol(Colon),
            '%' => Symbol(Percent),
            '^' => Symbol(Caret),
            '~' => Symbol(Tilde),

            // Whitespace.
            c if is_whitespace(c) => self.eat_whitespace(),

            // Number literal.
            '0'..='9' => self.eat_number(),

            // String literal.
            '"' => self.eat_string(),

            // Identifier or keyword.
            c if is_id_start(c) => self.eat_ident_or_keyword(c),

            _ => Unknown,
        };
        let mut span = self.update_span();
        span.end_line = self.line;
        span.end_pos = self.pos;
        Token::new(token_kand, span)
    }

    /// Update the span of the cursor and return the old span.
    fn update_span(&mut self) -> Span {
        let span = self.span;
        self.span = Span::new(self.line, self.pos, self.line, self.pos);
        span
    }

    /// Eat a comment.
    fn eat_comment(&mut self) -> TokenKind {
        self.eat_while(|c| c != '\n');
        Comment
    }

    /// Eat whitespace.
    fn eat_whitespace(&mut self) -> TokenKind {
        self.eat_while(crate::is_whitespace);
        Whitespace
    }

    /// Eat number literals.
    fn eat_number(&mut self) -> TokenKind {
        self.eat_while(|c| '0' <= c && c <= '9');
        Literal(LiteralKind::Integer)
    }

    /// Eat string literals.
    fn eat_string(&mut self) -> TokenKind {
        self.eat_while(|c| c != '"');
        self.bump();
        Literal(LiteralKind::String)
    }

    /// Eat identifiers and keywords.
    fn eat_ident_or_keyword(&mut self, c: char) -> TokenKind {
        let mut s = String::new();
        s.push(c);
        self.eat_while(|c| {
            let is_id_continue = is_id_continue(c);
            if is_id_continue {
                s.push(c);
            }
            is_id_continue
        });

        match s.as_str() {
            "let" => Keyword(Let),
            "fn" => Keyword(Fn),
            "if" => Keyword(If),
            "else" => Keyword(Else),
            "return" => Keyword(Return),
            "true" => Keyword(True),
            "false" => Keyword(False),
            "pub" => Keyword(Pub),
            "import" => Keyword(Import),
            "as" => Keyword(As),
            "extern" => Keyword(Extern),
            _ => Identifier,
        }
    }
}