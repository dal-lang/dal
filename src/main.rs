use dal::{
    codegen::{CodeGen, CodeGenBuildType, OutType},
    err_msg::ErrColor,
};

fn usage(cmd: &str, exit_code: i32) -> ! {
    println!("Usage: {} <command> [options]", cmd);
    println!("Commands:");
    println!("\tbuild\t\tCreate executable, object, or library from the source code");
    println!("\tversion\t\tPrint version information");
    println!("\thelp\t\tPrint this help message");
    println!("Command: build [options] <source file>");
    println!("\t--release\t\tBuild in release mode");
    println!("\t--static\t\tOutput will be linked statically");
    println!("\t--strip\t\t\tStrip debug symbols");
    println!("\t--export <exe|lib|obj>\tOverride output type");
    println!("\t--name <name>\t\tOverride output name");
    println!("\t--out <path>\t\tOverride output path");
    println!("\t--verbose\t\tPrint verbose output");
    println!("\t--color <auto|on|off>\t\tOverride color output");
    std::process::exit(exit_code);
}

fn version() -> ! {
    println!("DAL: v0.1.0");
    std::process::exit(0);
}

struct BuildOptions {
    in_file: String,
    out_file: String,
    out_name: String,
    release: bool,
    strip: bool,
    static_link: bool,
    verbose: bool,
    color: ErrColor,
    out_type: Option<OutType>,
}

impl BuildOptions {
    fn new() -> Self {
        Self {
            in_file: String::new(),
            out_file: String::new(),
            out_name: String::new(),
            release: false,
            strip: false,
            static_link: false,
            verbose: false,
            color: ErrColor::Auto,
            out_type: None,
        }
    }
}

fn build(args: Vec<String>) -> ! {
    let cwd = std::env::current_dir().unwrap();
    let cwd = cwd.as_path();

    if args.is_empty() {
        usage("dal", 1);
    }
    let mut opts = BuildOptions::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--release" => opts.release = true,
            "--static" => opts.static_link = true,
            "--strip" => opts.strip = true,
            "--verbose" => opts.verbose = true,
            "--color" => {
                i += 1;
                if i >= args.len() {
                    usage("dal", 1);
                }
                let color = &args[i];
                match color.as_str() {
                    "auto" => opts.color = ErrColor::Auto,
                    "on" => opts.color = ErrColor::On,
                    "off" => opts.color = ErrColor::Off,
                    _ => usage("dal", 1),
                }
            }
            "--export" => {
                i += 1;
                if i >= args.len() {
                    usage("dal", 1);
                }
                let out_type = &args[i];
                match out_type.as_str() {
                    "exe" => opts.out_type = Some(OutType::Exe),
                    "lib" => opts.out_type = Some(OutType::Lib),
                    "obj" => opts.out_type = Some(OutType::Obj),
                    _ => usage("dal", 1),
                }
            }
            "--name" => {
                i += 1;
                if i >= args.len() {
                    usage("dal", 1);
                }
                opts.out_name = args[i].clone();
            }
            "--out" => {
                i += 1;
                if i >= args.len() {
                    usage("dal", 1);
                }
                opts.out_file = args[i].clone();
            }
            _ => {
                if !arg.starts_with("--") {
                    if opts.in_file.is_empty() {
                        opts.in_file = arg.clone();
                    } else {
                        usage("dal", 1);
                    }
                } else {
                    usage("dal", 1);
                }
            }
        }
        i += 1;
    }

    if opts.in_file.is_empty() {
        usage("dal", 1);
    }

    let path = cwd.join(&opts.in_file);
    let root_source_dir = path.parent().unwrap().to_str().unwrap().to_string();
    let root_source_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let root_source_code;
    match std::fs::read_to_string(&opts.in_file) {
        Ok(code) => root_source_code = code,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }

    let mut g = CodeGen::new(root_source_dir);
    if opts.release {
        g.build_type = CodeGenBuildType::Release;
    } else {
        g.build_type = CodeGenBuildType::Debug;
    }
    g.strip_debug_symbols = opts.strip;
    g.is_static = opts.static_link;
    if opts.out_type.is_some() {
        g.out_type = opts.out_type.unwrap();
    }
    if !opts.out_name.is_empty() {
        g.root_out_name = opts.out_name;
    }
    g.verbose = opts.verbose;
    g.err_color = opts.color;
    g.add_root_code(root_source_name, root_source_code);

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
