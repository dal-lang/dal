// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

pub mod codegen;
pub mod compiler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrColor {
    Auto,
    Off,
    On,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrMsg {
    pub line_start: usize,
    pub col_start: usize,
    // pub line_end: usize,
    // pub col_end: usize,
    pub msg: String,
    pub path: String,
    pub src: String,
    pub line_offsets: Vec<usize>,
}

impl ErrMsg {
    pub fn print(&self, color: &ErrColor) -> ! {
        if color == &ErrColor::On || (color == &ErrColor::Auto && atty::is(atty::Stream::Stderr)) {
            eprintln!(
                "{}:{}:{}: {}",
                self.path, self.line_start, self.col_start, self.msg
            );
            eprintln!("{}", self.src);
            eprintln!(
                "{}{}{}",
                " ".repeat(self.col_start),
                "^".repeat(self.msg.len()),
                " ".repeat(self.src.len() - self.col_start - self.msg.len())
            );
        } else {
            eprintln!(
                "{}:{}:{}: {}",
                self.path, self.line_start, self.col_start, self.msg
            );
            eprintln!("{}", self.src);
            eprintln!(
                "{}{}{}",
                " ".repeat(self.col_start),
                "^".repeat(self.msg.len()),
                " ".repeat(self.src.len() - self.col_start - self.msg.len())
            );
        }
        std::process::exit(1);
    }
}
