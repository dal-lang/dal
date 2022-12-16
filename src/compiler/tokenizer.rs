// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    KwFalse,
    KwFn,
    KwIf,
    KwImport,
    KwLet,
    KwMut,
    KwPub,
    KwReturn,
    KwTrue,
    LBrace,
    LBracket,
    LParen,
    NumLit,
    Percent,
    Plus,
    RBrace,
    RBracket,
    RParen,
    Slash,
    Star,
    StrLit,
}

impl TokenKind {
    pub fn to_string(&self) -> String {
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
            TokenKind::CmpLte => "CmpLte".to_string(),
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
            TokenKind::KwFalse => "KwFalse".to_string(),
            TokenKind::KwFn => "KwFn".to_string(),
            TokenKind::KwIf => "KwIf".to_string(),
            TokenKind::KwImport => "KwImport".to_string(),
            TokenKind::KwLet => "KwLet".to_string(),
            TokenKind::KwMut => "KwMut".to_string(),
            TokenKind::KwPub => "KwPub".to_string(),
            TokenKind::KwReturn => "KwReturn".to_string(),
            TokenKind::KwTrue => "KwTrue".to_string(),
            TokenKind::LBrace => "LBrace".to_string(),
            TokenKind::LBracket => "LBracket".to_string(),
            TokenKind::LParen => "LParen".to_string(),
            TokenKind::NumLit => "NumLit".to_string(),
            TokenKind::Percent => "Percent".to_string(),
            TokenKind::Plus => "Plus".to_string(),
            TokenKind::RBrace => "RBrace".to_string(),
            TokenKind::RBracket => "RBracket".to_string(),
            TokenKind::RParen => "RParen".to_string(),
            TokenKind::Slash => "Slash".to_string(),
            TokenKind::Star => "Star".to_string(),
            TokenKind::StrLit => "StrLit".to_string(),
        }
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

pub fn print_tokens(src: &str, tokens: &Vec<RefCell<Token>>) {
    let tokens = tokens
        .iter()
        .map(|t| t.borrow().clone())
        .collect::<Vec<_>>();
    for token in tokens {
        println!(
            "{} {}",
            token.kind.to_string(),
            &src[token.start_pos..token.end_pos]
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tokenization {
    pub tokens: Vec<RefCell<Token>>,
    pub line_offsets: Vec<usize>,
    pub err: Option<String>,
    pub err_line: usize,
    pub err_col: usize,
}

impl Tokenization {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            line_offsets: Vec::new(),
            err: None,
            err_line: 0,
            err_col: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenizeState {
    Start,
    Error,
    Ident,
    Number,
    SawAssign,
    SawAt,
    SawAmp,
    SawBang,
    SawDash,
    SawGt,
    SawLt,
    SawPipe,
    SawSlash,
    String,
    Comment,
}

#[derive(Debug)]
struct Tokenize<'a> {
    buf: Vec<char>,
    pos: usize,
    state: TokenizeState,
    tokens: Vec<RefCell<Token>>,
    line: usize,
    col: usize,
    cur_tok: Option<*const RefCell<Token>>,
    out: &'a mut Tokenization,
}

impl<'a> Tokenize<'a> {
    fn start(buf: Vec<char>, out: &'a mut Tokenization) {
        let mut t = Self {
            buf,
            pos: 0,
            state: TokenizeState::Start,
            tokens: Vec::new(),
            line: 0,
            col: 0,
            cur_tok: None,
            out,
        };
        t.lex();
    }

    fn begin_token(&mut self, kind: TokenKind) {
        assert!(self.cur_tok.is_none());
        self.tokens.push(RefCell::new(Token::new(
            kind, self.pos, self.pos, self.line, self.col,
        )));
        self.cur_tok = Some(&self.tokens[self.tokens.len() - 1]);
    }

    fn finalize_token(&mut self, kind: TokenKind) {
        assert!(self.cur_tok.is_some());
        let tok = unsafe { &*self.cur_tok.unwrap() };
        tok.borrow_mut().kind = kind;
    }

    fn cancel_token(&mut self) {
        assert!(self.cur_tok.is_some());
        self.tokens.pop();
        self.cur_tok = None;
    }

    fn end_token(&mut self) {
        assert!(self.cur_tok.is_some());
        let tok = unsafe { &*self.cur_tok.unwrap() };
        tok.borrow_mut().end_pos = self.pos + 1;
        let tok_str = self.buf[tok.borrow().start_pos..tok.borrow().end_pos]
            .iter()
            .collect::<String>();
        match tok_str.as_str() {
            "fn" => tok.borrow_mut().kind = TokenKind::KwFn,
            "if" => tok.borrow_mut().kind = TokenKind::KwIf,
            "else" => tok.borrow_mut().kind = TokenKind::KwElse,
            "true" => tok.borrow_mut().kind = TokenKind::KwTrue,
            "false" => tok.borrow_mut().kind = TokenKind::KwFalse,
            "let" => tok.borrow_mut().kind = TokenKind::KwLet,
            "mut" => tok.borrow_mut().kind = TokenKind::KwMut,
            "pub" => tok.borrow_mut().kind = TokenKind::KwPub,
            "const" => tok.borrow_mut().kind = TokenKind::KwConst,
            "as" => tok.borrow_mut().kind = TokenKind::KwAs,
            "return" => tok.borrow_mut().kind = TokenKind::KwReturn,
            "@import" => tok.borrow_mut().kind = TokenKind::KwImport,
            "@asm" => tok.borrow_mut().kind = TokenKind::KwAsm,
            _ => {}
        }
        self.cur_tok = None;
    }

    fn is_special_kw(&self) -> (bool, Option<TokenKind>, String) {
        let cur_tok = unsafe { &*self.cur_tok.unwrap() };
        let tok_str = self.buf[cur_tok.borrow().start_pos..self.pos]
            .iter()
            .collect::<String>();
        match tok_str.as_str() {
            "@import" => (true, Some(TokenKind::KwImport), tok_str),
            "@asm" => (true, Some(TokenKind::KwAsm), tok_str),
            _ => (false, None, tok_str),
        }
    }

    fn error(&mut self, msg: String) {
        self.state = TokenizeState::Error;
        if self.cur_tok.is_some() {
            let cur_tok = unsafe { &*self.cur_tok.unwrap() };
            self.out.err_line = cur_tok.borrow().start_line;
            self.out.err_col = cur_tok.borrow().start_col;
        } else {
            self.out.err_line = self.line;
            self.out.err_col = self.col;
        }
        self.out.err = Some(msg);
    }

    fn lex(&mut self) {
        self.out.line_offsets.push(0);
        while self.pos < self.buf.len() {
            let c = self.buf[self.pos];
            match self.state {
                TokenizeState::Error => {}
                TokenizeState::Start => match c {
                    ' ' | '\n' => {}
                    '(' => {
                        self.begin_token(TokenKind::LParen);
                        self.end_token();
                    }
                    ')' => {
                        self.begin_token(TokenKind::RParen);
                        self.end_token();
                    }
                    '{' => {
                        self.begin_token(TokenKind::LBrace);
                        self.end_token();
                    }
                    '}' => {
                        self.begin_token(TokenKind::RBrace);
                        self.end_token();
                    }
                    '[' => {
                        self.begin_token(TokenKind::LBracket);
                        self.end_token();
                    }
                    ']' => {
                        self.begin_token(TokenKind::RBracket);
                        self.end_token();
                    }
                    ',' => {
                        self.begin_token(TokenKind::Comma);
                        self.end_token();
                    }
                    ':' => {
                        self.begin_token(TokenKind::Colon);
                        self.end_token();
                    }
                    '*' => {
                        self.begin_token(TokenKind::Star);
                        self.end_token();
                    }
                    '+' => {
                        self.begin_token(TokenKind::Plus);
                        self.end_token();
                    }
                    '%' => {
                        self.begin_token(TokenKind::Percent);
                        self.end_token();
                    }
                    '^' => {
                        self.begin_token(TokenKind::BitXor);
                        self.end_token();
                    }
                    '~' => {
                        self.begin_token(TokenKind::BitNot);
                        self.end_token();
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        self.begin_token(TokenKind::Ident);
                        self.state = TokenizeState::Ident;
                    }
                    '0'..='9' => {
                        self.begin_token(TokenKind::NumLit);
                        self.state = TokenizeState::Number;
                    }
                    '"' => {
                        self.begin_token(TokenKind::StrLit);
                        self.state = TokenizeState::String;
                    }
                    '<' => {
                        self.begin_token(TokenKind::CmpLt);
                        self.state = TokenizeState::SawLt;
                    }
                    '>' => {
                        self.begin_token(TokenKind::CmpGt);
                        self.state = TokenizeState::SawGt;
                    }
                    '=' => {
                        self.begin_token(TokenKind::Assign);
                        self.state = TokenizeState::SawAssign;
                    }
                    '!' => {
                        self.begin_token(TokenKind::Bang);
                        self.state = TokenizeState::SawBang;
                    }
                    '&' => {
                        self.begin_token(TokenKind::BitAnd);
                        self.state = TokenizeState::SawAmp;
                    }
                    '|' => {
                        self.begin_token(TokenKind::BitOr);
                        self.state = TokenizeState::SawPipe;
                    }
                    '-' => {
                        self.begin_token(TokenKind::Dash);
                        self.state = TokenizeState::SawDash;
                    }
                    '/' => {
                        self.begin_token(TokenKind::Slash);
                        self.state = TokenizeState::SawSlash;
                    }
                    '@' => {
                        self.begin_token(TokenKind::At);
                        self.state = TokenizeState::SawAt;
                    }
                    _ => {
                        self.error(format!("Unexpected character: {}", c));
                    }
                },
                TokenizeState::Ident => match c {
                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {}
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::Number => match c {
                    '0'..='9' => {}
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::String => match c {
                    '"' => {
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    _ => {}
                },
                TokenizeState::SawLt => match c {
                    '<' => {
                        self.finalize_token(TokenKind::BitShl);
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    '=' => {
                        self.finalize_token(TokenKind::CmpLte);
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::SawGt => match c {
                    '>' => {
                        self.finalize_token(TokenKind::BitShr);
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    '=' => {
                        self.finalize_token(TokenKind::CmpGte);
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::SawAssign => match c {
                    '=' => {
                        self.finalize_token(TokenKind::CmpEq);
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::SawBang => match c {
                    '=' => {
                        self.finalize_token(TokenKind::CmpNeq);
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::SawAmp => match c {
                    '&' => {
                        self.finalize_token(TokenKind::BoolAnd);
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::SawPipe => match c {
                    '|' => {
                        self.finalize_token(TokenKind::BoolOr);
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::SawDash => match c {
                    '>' => {
                        self.finalize_token(TokenKind::Arrow);
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::SawSlash => match c {
                    '/' => {
                        self.state = TokenizeState::Comment;
                        self.cancel_token();
                    }
                    _ => {
                        self.pos -= 1;
                        self.end_token();
                        self.state = TokenizeState::Start;
                    }
                },
                TokenizeState::Comment => match c {
                    '\n' => {
                        self.state = TokenizeState::Start;
                    }
                    _ => {}
                },
                TokenizeState::SawAt => match c {
                    'a'..='z' => {}
                    _ => {
                        let (special, kind, str) = self.is_special_kw();
                        if special {
                            self.finalize_token(kind.unwrap());
                            self.pos -= 1;
                            self.end_token();
                            self.state = TokenizeState::Start;
                        } else {
                            self.error(format!("Unexpected keyword: {}", str));
                        }
                    }
                },
            }
            if c == '\n' {
                self.out.line_offsets.push(self.pos + 1);
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }

        match self.state {
            TokenizeState::Ident
            | TokenizeState::Number
            | TokenizeState::SawDash
            | TokenizeState::SawPipe
            | TokenizeState::SawAmp
            | TokenizeState::SawAssign
            | TokenizeState::SawBang
            | TokenizeState::SawLt
            | TokenizeState::SawGt => {
                self.end_token();
            }
            TokenizeState::SawSlash | TokenizeState::SawAt => {
                self.error("Unexpected end of file".to_string());
            }
            TokenizeState::String => {
                self.error("Unterminated string".to_string());
            }
            _ => {}
        }

        if self.state != TokenizeState::Error {
            self.pos -= 1;
            self.begin_token(TokenKind::Eof);
            self.end_token();
            assert!(self.cur_tok.is_none());
        }

        self.out.tokens = self.tokens.clone();
    }
}

pub fn tokenize(src: &str, out: &mut Tokenization) {
    let mut buf = Vec::new();
    for c in src.chars() {
        buf.push(c);
    }
    Tokenize::start(buf, out);
}
