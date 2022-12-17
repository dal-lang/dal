// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use crate::codegen::analyze::ImportTableEntry;

use super::tokenizer::Token;
use std::cell::RefCell;

pub fn parse(src: &str, tokens: Vec<RefCell<Token>>, owner: &mut ImportTableEntry) {
    unreachable!();
}
