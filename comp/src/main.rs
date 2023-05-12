use dal::{
    cli::{App, ArgKind, CliContext, Command},
    lexer::Lexer,
    span::SourceFile,
};

static VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn compile_handler(ctx: &CliContext) -> i32 {
    let output = ctx.get_string("o").unwrap_or("a.out");
    let emit = ctx.get_string("emit").unwrap_or("exe");

    let free_len = ctx.get_frees().len();
    if free_len == 0 {
        println!("No target specified");
        return 1;
    } else if free_len > 1 {
        println!("Too many targets specified");
        return 1;
    }

    let target = &ctx.get_frees()[0];

    let file = SourceFile::new(target);

    let mut lexer = Lexer::new(&file);

    let tokens = lexer.lex();
    dbg!(tokens);

    return 0;
}

fn main() {
    let mut app = App::new("dal", "Compiler for dal programming language", VERSION);

    Command::new()
        .name("compile")
        .desc("Compile target into executable, object, or library")
        .usage("build [options] <target>")
        .arg(|arg| {
            arg.name("output").short("o").desc("Override output destination").kind(ArgKind::String)
        })
        .arg(|arg| {
            arg.name("emit")
                .desc("Specify output type. default (exe)")
                .hint("[exe|lib|obj|asm|llvm-ir|wasm]")
                .kind(ArgKind::String)
        })
        .handler(compile_handler)
        .build(&mut app);

    app.parse();
}
