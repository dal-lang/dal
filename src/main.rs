use dal::{
    lexer::{self, print_tokens},
    parser,
};

fn usage(cmd: &str, exit_code: i32) -> ! {
    println!("Usage: {} <command> [options]", cmd);
    println!("Commands:");
    println!("\tversion\t\tPrint version information");
    std::process::exit(exit_code);
}

fn version() -> ! {
    println!("DAL: v0.1.0");
    std::process::exit(0);
}

fn build(args: Vec<String>) -> ! {
    if args.len() == 0 {
        println!("No input files");
        std::process::exit(1);
    }

    let f = std::fs::read_to_string(&args[0]);
    if f.is_err() {
        println!("Failed to read file: {}", args[0]);
        std::process::exit(1);
    }

    let src = f.unwrap();

    let mut out = lexer::Tokenization::new();
    lexer::tokenize(&src, &mut out);
    if out.errors.is_some() {
        println!("Failed to tokenize file: {}", args[0]);
        std::process::exit(1);
    }
    print_tokens(&src, &out.tokens);

    let ast = parser::parse(&src, out.tokens.clone());
    println!("{}", ast);

    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage(&args[0], 1);
    }

    let cmd = &args[1];
    match cmd.as_str() {
        "version" | "-v" | "--version" => version(),
        "build" => build(args[2..].to_vec()),
        _ => usage(&args[0], 1),
    }
}
