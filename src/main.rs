// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use dal::{
    codegen::analyze::ImportTableEntry,
    compiler::{
        parser::parse,
        tokenizer::{tokenize, Tokenization},
    },
    ErrColor,
};

fn usage(arg0: &str, exit_code: i32) -> ! {
    println!("Usage: {} [command] [options]", arg0);
    println!("Commands:");
    println!("  build\t\tCompile the project");
    println!("  version\tShow the version");
    println!("  help\t\tShow this help");
    println!("Command: build target");
    println!("  --release\t\tCompile the project in release mode");
    println!("  --static\t\tThe project will be statically linked");
    println!("  --strip\t\tStrip the binary");
    println!("  --color [auto|on|off]\tColorize the output");

    std::process::exit(exit_code);
}

fn version() -> ! {
    println!("dal 0.1.0");
    std::process::exit(0);
}

fn build(src_path: &str) -> ! {
    let src = std::fs::read_to_string(src_path).unwrap();
    let mut tok_out = Tokenization::new();
    tokenize(&src, &mut tok_out);
    if tok_out.err.is_some() {
        println!("Error: {}", tok_out.err.unwrap());
        std::process::exit(1);
    }

    let mut import_table_enty = ImportTableEntry::new(src_path, &src, tok_out.line_offsets);
    let ast = parse(&src, tok_out.tokens, &mut import_table_enty, ErrColor::Auto);
    ast.print_ast(0);
    // println!("{:#?}", ast);
    std::process::exit(0);
}

fn main() -> ! {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage(&args[0], 1);
    }

    match args[1].as_str() {
        "build" => {
            let mut target = None;
            if args.len() > 2 {
                target = Some(args[2].clone());
            }
            if target.is_none() {
                usage(&args[0], 1);
            }
            build(&target.unwrap());
        }
        "version" => version(),
        "help" => usage(&args[0], 0),
        _ => usage(&args[0], 1),
    }
}
