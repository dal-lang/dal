use self::token::{Token, TokenKind};
use crate::span::{SourceFile, Span};
use std::{process::exit, str::Chars};

pub mod token;

/// Lexer for the language.
pub struct Lexer<'a> {
    /// The source file.
    source: &'a SourceFile,
    /// The characters of the source file.
    chars: Chars<'a>,
    /// Resulting tokens.
    tokens: Vec<Token>,
    /// End of file token.
    eof: Token,
    /// Last position of the lexer.
    lo: usize,
    /// Current position of the lexer.
    hi: usize,
}

const EOF_CHAR: char = '\0';

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\n'
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_id_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_id_continue(c: char) -> bool {
    is_id_start(c) || is_digit(c)
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer.
    pub fn new(source: &'a SourceFile) -> Lexer<'a> {
        Lexer {
            source,
            chars: source.content.chars(),
            tokens: Vec::new(),
            eof: Token::new(token::TokenKind::Eof, source.eof_span()),
            lo: 0,
            hi: 0,
        }
    }

    /// Bumps the lexer to the next character.
    fn bump(&mut self) -> char {
        let c = self.chars.next().unwrap_or(EOF_CHAR);
        if c != EOF_CHAR {
            self.hi += 1;
        }
        c
    }

    /// Peeks the next character without bumping the lexer.
    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Peeks the second character without bumping the lexer.
    fn second(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or(EOF_CHAR)
    }

    /// Eats all the characters that satisfy the predicate or until the end of
    /// file.
    fn eat_while<F>(&mut self, mut f: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut s = String::new();
        while f(self.first()) && self.first() != EOF_CHAR {
            s.push(self.bump());
        }
        s
    }

    /// Creates a span from the last position of the lexer to the current
    /// position and updates the last position.
    fn mk_span(&mut self) -> Span {
        let sp = Span::new(self.lo, self.hi);
        self.lo = self.hi;
        sp
    }

    /// Gets the next token.
    fn next_token(&mut self) -> Token {
        let c = match self.bump() {
            EOF_CHAR => return self.eof.clone(),
            c => c,
        };

        let kind = match c {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            ',' => TokenKind::Comma,
            '.' => {
                if self.first() == '.' && self.second() == '.' {
                    self.bump();
                    self.bump();
                    TokenKind::Ellipsis
                } else {
                    TokenKind::Dot
                }
            }
            '-' => {
                if self.first() == '>' {
                    self.bump();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            '+' => TokenKind::Plus,
            ';' => TokenKind::Semicolon,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '^' => TokenKind::Caret,
            ':' => TokenKind::Colon,
            '?' => TokenKind::Question,
            '!' => {
                if self.first() == '=' {
                    self.bump();
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '=' => {
                if self.first() == '=' {
                    self.bump();
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '>' => {
                if self.first() == '>' {
                    self.bump();
                    TokenKind::ShiftRight
                } else if self.first() == '=' {
                    self.bump();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '<' => {
                if self.first() == '<' {
                    self.bump();
                    TokenKind::ShiftLeft
                } else if self.first() == '=' {
                    self.bump();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '@' => TokenKind::At,
            c if is_whitespace(c) => {
                self.eat_while(|c| is_whitespace(c));
                return self.next_token();
            }
            c if is_digit(c) => {
                let s = self.eat_while(|c| is_digit(c));
                TokenKind::Number(format!("{}{}", c, s))
            }
            c if is_id_start(c) => {
                let s = self.eat_while(|c| is_id_continue(c));
                let s = format!("{}{}", c, s);
                match s.as_str() {
                    "module" => TokenKind::Module,
                    "import" => TokenKind::Import,
                    "extern" => TokenKind::Extern,
                    "pub" => TokenKind::Pub,
                    "fn" => TokenKind::Fn,
                    "let" => TokenKind::Let,
                    "const" => TokenKind::Const,
                    "mut" => TokenKind::Mut,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "return" => TokenKind::Return,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "as" => TokenKind::As,
                    _ => TokenKind::Identifier(s),
                }
            }
            '"' => {
                let s = self.eat_while(|c| c != '"');
                self.bump();
                TokenKind::String(s)
            }
            _ => TokenKind::Unknown,
        };

        let tok = Token::new(kind, self.mk_span());
        self.tokens.push(tok.clone());
        tok
    }

    /// Lexes the source file.
    pub fn lex(&mut self) -> Vec<Token> {
        loop {
            let tok = self.next_token();
            if tok.kind == TokenKind::Eof {
                break;
            }

            if tok.kind == TokenKind::Unknown {
                let str = self.source.content[tok.span.lo..tok.span.hi].to_string();
                println!("Unknown token: {}", str);
                exit(1);
            }
        }

        self.tokens.clone()
    }
}
