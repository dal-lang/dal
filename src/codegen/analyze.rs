// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use std::cell::RefCell;

use crate::compiler::ast::Node;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportTableEntry {
    pub root: Option<RefCell<Node>>,
    pub path: String,
    pub src_code: String,
    pub line_offsets: Vec<usize>,
}

impl ImportTableEntry {
    pub fn new(path: &str, src_code: &str, line_offsets: Vec<usize>) -> Self {
        Self {
            root: None,
            path: path.to_string(),
            src_code: src_code.to_string(),
            line_offsets,
        }
    }
}
