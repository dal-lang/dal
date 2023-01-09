use crate::KeywordKind::*;
use crate::SymbolKind::*;
use crate::TokenKind;
use crate::TokenKind::*;
use crate::{LiteralKind, Token};
use std::fmt::Formatter;

/// Get the string representation of a token from the source code.
pub fn get_token_string<'a>(source: &'a str, token: &Token) -> &'a str {
    let start = token.span.start_pos;
    let end = token.span.end_pos;
    &source[start..end]
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword(k) => match k {
                Let => write!(f, "Let"),
                Fn => write!(f, "Fn"),
                If => write!(f, "If"),
                Else => write!(f, "Else"),
                Return => write!(f, "Return"),
                True => write!(f, "True"),
                False => write!(f, "False"),
                Pub => write!(f, "Pub"),
                Import => write!(f, "Import"),
                As => write!(f, "As"),
                Extern => write!(f, "Extern"),
                Const => write!(f, "Const"),
                Mut => write!(f, "Mut"),
            },
            Literal(l) => match l {
                LiteralKind::Integer => write!(f, "Integer"),
                LiteralKind::String => write!(f, "String"),
            },
            Identifier => write!(f, "Identifier"),
            Symbol(s) => match s {
                Plus => write!(f, "Plus"),
                Minus => write!(f, "Minus"),
                Asterisk => write!(f, "Asterisk"),
                Slash => write!(f, "Slash"),
                Bang => write!(f, "Bang"),
                Equal => write!(f, "Equal"),
                EqualEqual => write!(f, "EqualEqual"),
                BangEqual => write!(f, "BangEqual"),
                Less => write!(f, "Less"),
                Greater => write!(f, "Greater"),
                LessEqual => write!(f, "LessEqual"),
                GreaterEqual => write!(f, "GreaterEqual"),
                LeftShift => write!(f, "LeftShift"),
                RightShift => write!(f, "RightShift"),
                LeftParen => write!(f, "LeftParen"),
                RightParen => write!(f, "RightParen"),
                LeftBrace => write!(f, "LeftBrace"),
                RightBrace => write!(f, "RightBrace"),
                LeftBracket => write!(f, "LeftBracket"),
                RightBracket => write!(f, "RightBracket"),
                Comma => write!(f, "Comma"),
                Dot => write!(f, "Dot"),
                DotDot => write!(f, "DotDot"),
                Colon => write!(f, "Colon"),
                Semi => write!(f, "Semi"),
                Percent => write!(f, "Percent"),
                Caret => write!(f, "Caret"),
                And => write!(f, "And"),
                AndAnd => write!(f, "AndAnd"),
                Or => write!(f, "Or"),
                OrOr => write!(f, "OrOr"),
                Tilde => write!(f, "Tilde"),
                Arrow => write!(f, "Arrow"),
            },
            Comment => write!(f, "Comment"),
            Whitespace => write!(f, "Whitespace"),
            Unknown => write!(f, "Unknown"),
            Eof => write!(f, "Eof"),
        }
    }
}

/// Print a token to the console.
pub fn print_token(token: &Token, source: &str) {
    let token_string = get_token_string(source, token);
    println!("{}: {}", token.kind, token_string);
}
