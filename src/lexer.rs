// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Arrow,
    Assign,
    At,
    Bang,
    BitAnd,
    BitNot,
    BitOr,
    BitShl,
    BitShr,
    BitXor,
    BoolAnd,
    BoolOr,
    CmpEq,
    CmpGt,
    CmpGte,
    CmpLt,
    CmpLte,
    CmpNeq,
    Colon,
    Comma,
    Dash,
    Eof,
    Ident,
    KwAs,
    KwAsm,
    KwConst,
    KwElse,
    KwExtern,
    KwFalse,
    KwFn,
    KwIf,
    KwImport,
    KwLet,
    KwMut,
    KwPub,
    KwReturn,
    KwTrue,
    KwVoid,
    LBrace,
    LBracket,
    LParen,
    NumLit,
    Percent,
    Plus,
    RBrace,
    RBracket,
    RParen,
    Semicolon,
    Slash,
    Star,
    StrLit,
    Comment,
    Dot,
    DotDot,
}

impl TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Arrow => "Arrow".to_string(),
            TokenKind::Assign => "Assign".to_string(),
            TokenKind::At => "At".to_string(),
            TokenKind::Bang => "Bang".to_string(),
            TokenKind::BitAnd => "BitAnd".to_string(),
            TokenKind::BitNot => "BitNot".to_string(),
            TokenKind::BitOr => "BitOr".to_string(),
            TokenKind::BitShl => "BitShl".to_string(),
            TokenKind::BitShr => "BitShr".to_string(),
            TokenKind::BitXor => "BitXor".to_string(),
            TokenKind::BoolAnd => "BoolAnd".to_string(),
            TokenKind::BoolOr => "BoolOr".to_string(),
            TokenKind::CmpEq => "CmpEq".to_string(),
            TokenKind::CmpGt => "CmpGt".to_string(),
            TokenKind::CmpGte => "CmpGte".to_string(),
            TokenKind::CmpLt => "CmpLt".to_string(),
            TokenKind::CmpLte => "cmpLte".to_string(),
            TokenKind::CmpNeq => "CmpNeq".to_string(),
            TokenKind::Colon => "Colon".to_string(),
            TokenKind::Comma => "Comma".to_string(),
            TokenKind::Dash => "Dash".to_string(),
            TokenKind::Eof => "Eof".to_string(),
            TokenKind::Ident => "Ident".to_string(),
            TokenKind::KwAs => "KwAs".to_string(),
            TokenKind::KwAsm => "KwAsm".to_string(),
            TokenKind::KwConst => "KwConst".to_string(),
            TokenKind::KwElse => "KwElse".to_string(),
            TokenKind::KwExtern => "KwExtern".to_string(),
            TokenKind::KwFalse => "KwFalse".to_string(),
            TokenKind::KwFn => "KwFn".to_string(),
            TokenKind::KwIf => "KwIf".to_string(),
            TokenKind::KwImport => "KwImport".to_string(),
            TokenKind::KwLet => "KwLet".to_string(),
            TokenKind::KwMut => "KwMut".to_string(),
            TokenKind::KwPub => "KwPub".to_string(),
            TokenKind::KwReturn => "KwReturn".to_string(),
            TokenKind::KwTrue => "KwTrue".to_string(),
            TokenKind::KwVoid => "KwVoid".to_string(),
            TokenKind::LBrace => "LBrace".to_string(),
            TokenKind::LBracket => "LBracket".to_string(),
            TokenKind::LParen => "LParen".to_string(),
            TokenKind::NumLit => "NumLit".to_string(),
            TokenKind::Percent => "Percent".to_string(),
            TokenKind::Plus => "Plus".to_string(),
            TokenKind::RBrace => "RBrace".to_string(),
            TokenKind::RBracket => "RBracket".to_string(),
            TokenKind::RParen => "RParen".to_string(),
            TokenKind::Semicolon => "Semicolon".to_string(),
            TokenKind::Slash => "Slash".to_string(),
            TokenKind::Star => "Star".to_string(),
            TokenKind::StrLit => "StrLit".to_string(),
            TokenKind::Comment => "Comment".to_string(),
            TokenKind::Dot => "Dot".to_string(),
            TokenKind::DotDot => "DotDot".to_string(),
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub start_pos: usize,
    pub end_pos: usize,
    pub start_line: usize,
    pub start_col: usize,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        start_pos: usize,
        end_pos: usize,
        start_line: usize,
        start_col: usize,
    ) -> Self {
        Self {
            kind,
            start_pos,
            end_pos,
            start_line,
            start_col,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tokenization {
    pub tokens: Vec<Token>,
    pub line_offsets: Vec<usize>,
    pub errors: Option<String>,
    pub error_line: usize,
    pub error_col: usize,
}

impl Tokenization {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            line_offsets: Vec::new(),
            errors: None,
            error_line: 0,
            error_col: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TokenizerState {
    Start,
    Ident,
    NumLit,
    StrLit,
    CmpLt,
    CmpGt,
    Assign,
    Bang,
    BitAnd,
    BitOr,
    Dash,
    Slash,
    At,
    Error,
    Comment,
    Dot,
}

#[derive(Debug)]
struct Tokenizer<'a> {
    out: &'a mut Tokenization,
    pos: usize,
    line: usize,
    col: usize,
    src: Vec<char>,
    state: TokenizerState,
    token: Option<Token>,
}

impl<'a> Tokenizer<'a> {
    fn start(src: Vec<char>, out: &'a mut Tokenization) {
        let mut t = Self {
            out,
            pos: 0,
            line: 0,
            col: 0,
            src,
            state: TokenizerState::Start,
            token: None,
        };
        t.lex();
    }

    fn lex(&mut self) {
        self.out.line_offsets.push(0);
        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            match self.state {
                TokenizerState::Error => {}
                TokenizerState::Start => self.lex_start(c),
                TokenizerState::Ident => self.ident(c),
                TokenizerState::NumLit => self.num_lit(c),
                TokenizerState::StrLit => self.str_lit(c),
                TokenizerState::CmpLt => self.cmp_lt(c),
                TokenizerState::CmpGt => self.cmp_gt(c),
                TokenizerState::Assign => self.assign(c),
                TokenizerState::Bang => self.bang(c),
                TokenizerState::BitAnd => self.bit_and(c),
                TokenizerState::BitOr => self.bit_or(c),
                TokenizerState::Dash => self.dash(c),
                TokenizerState::Slash => self.slash(c),
                TokenizerState::At => self.at(c),
                TokenizerState::Comment => self.comment(c),
                TokenizerState::Dot => self.dot(c),
            }
            if c == '\n' {
                self.line += 1;
                self.col = 0;
                self.out.line_offsets.push(self.pos + 1);
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }

        match self.state {
            TokenizerState::Ident
            | TokenizerState::NumLit
            | TokenizerState::Dash
            | TokenizerState::BitOr
            | TokenizerState::BitAnd
            | TokenizerState::Assign
            | TokenizerState::CmpGt
            | TokenizerState::CmpLt => {
                self.end_token();
            }
            TokenizerState::Slash | TokenizerState::At => {
                self.error("Unexpected end of file".to_string());
            }
            TokenizerState::StrLit => {
                self.error("Unterminated string literal".to_string());
            }
            _ => {}
        }

        if self.state != TokenizerState::Error {
            self.pos -= 1;
            self.begin_token(TokenKind::Eof);
            self.end_token();
        }
    }

    fn lex_start(&mut self, c: char) {
        match c {
            ' ' | '\n' => {}
            '(' => self.add_token(TokenKind::LParen),
            ')' => self.add_token(TokenKind::RParen),
            '{' => self.add_token(TokenKind::LBrace),
            '}' => self.add_token(TokenKind::RBrace),
            '[' => self.add_token(TokenKind::LBracket),
            ']' => self.add_token(TokenKind::RBracket),
            ',' => self.add_token(TokenKind::Comma),
            ':' => self.add_token(TokenKind::Colon),
            '*' => self.add_token(TokenKind::Star),
            '+' => self.add_token(TokenKind::Plus),
            '%' => self.add_token(TokenKind::Percent),
            '^' => self.add_token(TokenKind::BitXor),
            '~' => self.add_token(TokenKind::BitNot),
            ';' => self.add_token(TokenKind::Semicolon),
            'a'..='z' | 'A'..='Z' | '_' => {
                self.state = TokenizerState::Ident;
                self.begin_token(TokenKind::Ident);
            }
            '0'..='9' => {
                self.state = TokenizerState::NumLit;
                self.begin_token(TokenKind::NumLit);
            }
            '"' => {
                self.state = TokenizerState::StrLit;
                self.begin_token(TokenKind::StrLit);
            }
            '<' => {
                self.state = TokenizerState::CmpLt;
                self.begin_token(TokenKind::CmpLt);
            }
            '>' => {
                self.state = TokenizerState::CmpGt;
                self.begin_token(TokenKind::CmpGt);
            }
            '=' => {
                self.state = TokenizerState::Assign;
                self.begin_token(TokenKind::Assign);
            }
            '!' => {
                self.state = TokenizerState::Bang;
                self.begin_token(TokenKind::Bang);
            }
            '&' => {
                self.state = TokenizerState::BitAnd;
                self.begin_token(TokenKind::BitAnd);
            }
            '|' => {
                self.state = TokenizerState::BitOr;
                self.begin_token(TokenKind::BitOr);
            }
            '-' => {
                self.state = TokenizerState::Dash;
                self.begin_token(TokenKind::Dash);
            }
            '/' => {
                self.state = TokenizerState::Slash;
                self.begin_token(TokenKind::Slash);
            }
            '@' => {
                self.state = TokenizerState::At;
                self.begin_token(TokenKind::At);
            }
            '.' => {
                self.state = TokenizerState::Dot;
                self.begin_token(TokenKind::Dot);
            }
            _ => {
                self.error(format!("Unexpected character '{}'", c));
            }
        }
    }

    fn error(&mut self, msg: String) {
        self.state = TokenizerState::Error;
        if let Some(tok) = &self.token {
            self.out.error_line = tok.start_line;
            self.out.error_col = tok.start_col;
        } else {
            self.out.error_line = self.line;
            self.out.error_col = self.col;
        }
        self.out.errors = Some(msg);
    }

    fn begin_token(&mut self, kind: TokenKind) {
        assert!(self.token.is_none());
        self.token = Some(Token::new(kind, self.pos, self.pos, self.line, self.col));
    }

    fn end_token(&mut self) {
        assert!(self.token.is_some());
        let mut tok = self.token.take().unwrap();
        tok.end_pos = self.pos + 1;
        let tok_str = self.src[tok.start_pos..tok.end_pos]
            .iter()
            .collect::<String>();
        match tok_str.as_str() {
            "fn" => tok.kind = TokenKind::KwFn,
            "if" => tok.kind = TokenKind::KwIf,
            "else" => tok.kind = TokenKind::KwElse,
            "true" => tok.kind = TokenKind::KwTrue,
            "false" => tok.kind = TokenKind::KwFalse,
            "let" => tok.kind = TokenKind::KwLet,
            "mut" => tok.kind = TokenKind::KwMut,
            "pub" => tok.kind = TokenKind::KwPub,
            "const" => tok.kind = TokenKind::KwConst,
            "as" => tok.kind = TokenKind::KwAs,
            "return" => tok.kind = TokenKind::KwReturn,
            "void" => tok.kind = TokenKind::KwVoid,
            "extern" => tok.kind = TokenKind::KwExtern,
            "@import" => tok.kind = TokenKind::KwImport,
            "@asm" => tok.kind = TokenKind::KwAsm,
            _ => {}
        }
        self.out.tokens.push(tok);
        self.token = None;
        self.state = TokenizerState::Start;
    }

    fn end_with(&mut self, kind: TokenKind) {
        assert!(self.token.is_some());
        let mut tok = self.token.take().unwrap();
        tok.end_pos = self.pos + 1;
        tok.kind = kind;
        self.out.tokens.push(tok);
        self.token = None;
        self.state = TokenizerState::Start;
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.begin_token(kind);
        self.end_token();
    }

    fn ident(&mut self, c: char) {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {}
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn num_lit(&mut self, c: char) {
        match c {
            '0'..='9' => {}
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn str_lit(&mut self, c: char) {
        match c {
            '"' => {
                self.end_token();
            }
            _ => {}
        }
    }

    fn cmp_lt(&mut self, c: char) {
        match c {
            '=' => {
                self.end_with(TokenKind::CmpLte);
            }
            '<' => {
                self.end_with(TokenKind::BitShl);
            }
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn cmp_gt(&mut self, c: char) {
        match c {
            '=' => {
                self.end_with(TokenKind::CmpGte);
            }
            '>' => {
                self.end_with(TokenKind::BitShr);
            }
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn assign(&mut self, c: char) {
        match c {
            '=' => {
                self.end_with(TokenKind::CmpEq);
            }
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn bang(&mut self, c: char) {
        match c {
            '=' => {
                self.end_with(TokenKind::CmpNeq);
            }
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn bit_and(&mut self, c: char) {
        match c {
            '&' => {
                self.end_with(TokenKind::BoolAnd);
            }
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn bit_or(&mut self, c: char) {
        match c {
            '|' => {
                self.end_with(TokenKind::BoolOr);
            }
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn dash(&mut self, c: char) {
        match c {
            '>' => {
                self.end_with(TokenKind::Arrow);
            }
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn slash(&mut self, c: char) {
        match c {
            '/' => {
                self.state = TokenizerState::Comment;
            }
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }

    fn comment(&mut self, c: char) {
        match c {
            '\n' => {
                self.end_with(TokenKind::Comment);
            }
            _ => {}
        }
    }

    fn at(&mut self, c: char) {
        match c {
            'a'..='z' => {}
            _ => {
                self.pos -= 1;
                assert!(self.token.is_some());
                let mut tok = self.token.take().unwrap();
                tok.end_pos = self.pos + 1;
                let tok_str = self.src[tok.start_pos..tok.end_pos]
                    .iter()
                    .collect::<String>();
                match tok_str.as_str() {
                    "@import" => tok.kind = TokenKind::KwImport,
                    "@asm" => tok.kind = TokenKind::KwAsm,
                    _ => {
                        self.error(format!("Unknown token: {}", tok_str));
                    }
                }
                self.out.tokens.push(tok);
                self.token = None;
                self.state = TokenizerState::Start;
            }
        }
    }

    fn dot(&mut self, c: char) {
        match c {
            '.' => {
                self.end_with(TokenKind::DotDot);
            }
            _ => {
                self.pos -= 1;
                self.end_token();
            }
        }
    }
}

pub fn tokenize(src: &str, out: &mut Tokenization) {
    let mut chars = src.chars().collect::<Vec<char>>();
    if chars.last() != Some(&'\n') {
        chars.push('\n');
    }
    Tokenizer::start(chars, out);
}

pub fn print_tokens(src: &str, tokens: &[Token]) {
    for token in tokens {
        let tok_str = &src[token.start_pos..token.end_pos];
        println!("{} {}", token.kind, tok_str);
    }
}
