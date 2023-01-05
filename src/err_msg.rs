// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

pub static RED: &str = "\x1b[31;1m";
pub static WHITE: &str = "\x1b[37;1m";
pub static GREEN: &str = "\x1b[32;1m";
pub static RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrColor {
    Off,
    On,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrMsg {
    pub line_start: usize,
    pub line_end: usize,
    pub col_start: usize,
    pub col_end: usize,
    pub msg: String,
    pub path: String,
    pub src: String,
    pub line_offsets: Vec<usize>,
}

impl ErrMsg {
    pub fn new() -> Self {
        Self {
            line_start: 0,
            line_end: 0,
            col_start: 0,
            col_end: 0,
            msg: String::new(),
            path: String::new(),
            src: String::new(),
            line_offsets: Vec::new(),
        }
    }

    pub fn print(&self, color: ErrColor) -> ! {
        if color == ErrColor::On || (color == ErrColor::Auto && atty::is(atty::Stream::Stderr)) {
            eprintln!(
                "{}{}:{}:{} {}Error: {}{}{}\n",
                WHITE,
                self.path,
                self.line_start + 1,
                self.col_start + 1,
                RED,
                WHITE,
                self.msg,
                RESET
            );

            let line_start_offset = self.line_offsets[self.line_start];
            let end_line = self.line_start + 1;
            let line_end_offset = if end_line >= self.line_offsets.len() {
                self.src.len()
            } else {
                self.line_offsets[end_line]
            };

            // c++: fwrite(buf_ptr(err->source) + line_start_offset, 1, line_end_offset - line_start_offset - 1, stderr);
            let src = &self.src[line_start_offset..line_end_offset - 1];
            eprintln!("{}", src);
            for _ in 0..self.col_start {
                eprint!(" ");
            }
            eprintln!("^");
        } else {
            eprintln!(
                "{}:{}:{} Error: {}",
                self.path,
                self.line_start + 1,
                self.col_start + 1,
                self.msg
            );
        }
        // exit error
        std::process::exit(1);
    }
}
