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
        std::process::exit(1);
    }
}
