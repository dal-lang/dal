// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use dal::compiler::tokenizer::{print_tokens, tokenize, Tokenization};

fn main() {
    let src = include_str!("../example/hello.dal");
    let mut out = Tokenization::new();
    tokenize(src, &mut out);
    if let Some(err) = out.err {
        println!("Error: {}", err);
    } else {
        print_tokens(src, &out.tokens);
    }
}
