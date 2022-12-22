// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use dal::{
    codegen::analyze::ImportTableEntry,
    compiler::{
        parser::parse,
        tokenizer::{print_tokens, tokenize, Tokenization},
    },
    ErrColor,
};

fn main() {
    let src = include_str!("../example/hello.dal");
    let mut out = Tokenization::new();
    tokenize(src, &mut out);
    if let Some(err) = out.err {
        println!("Error: {}", err);
    } else {
        print_tokens(src, &out.tokens);
    }

    let mut import_table_entry = ImportTableEntry::new("./", src, out.line_offsets);
    let ast = parse(src, out.tokens, &mut import_table_entry, ErrColor::Auto);
    ast.print_ast(0);
}
