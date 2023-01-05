use std::{io, process::Command};

fn llvm_config(arg: &str) -> String {
    let output =
        Command::new("llvm-config")
            .arg(arg)
            .arg("--link-static")
            .output()
            .and_then(|output| {
                if output.stdout.is_empty() {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        "llvm-config-15 returned no output",
                    ))
                } else {
                    Ok(String::from_utf8(output.stdout)
                        .expect("llvm-config-15 returned invalid UTF-8"))
                }
            });
    output.expect("failed to run llvm-config-15")
}

fn get_link_libraries() -> Vec<String> {
    llvm_config("--libnames")
        .split(&[' ', '\n'] as &[char])
        .filter(|s| !s.is_empty())
        .map(|name| {
            assert!(
                name.starts_with("lib") && name.ends_with(".a"),
                "unexpected library name: {}",
                name
            );
            name[3..name.len() - 2].to_string()
        })
        .collect()
}

fn llvm_cflags() -> String {
    llvm_config("--cflags")
        .split(&[' ', '\n'][..])
        .filter(|word| !word.starts_with("-W"))
        .collect::<Vec<_>>()
        .join(" ")
}

#[allow(dead_code)]
fn llvm_cxxflags() -> String {
    llvm_config("--cxxflags")
        .split(&[' ', '\n'][..])
        .filter(|word| !word.starts_with("-W"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    std::env::set_var("CFLAGS", llvm_cflags());
    cc::Build::new()
        .file("llvm_wrapper/target.c")
        .compile("targetwrapper");

    let libdir = llvm_config("--libdir");

    println!("cargo:libdir={}", libdir);

    println!("cargo:rustc-link-search=native={}", libdir);
    for lib in get_link_libraries() {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    println!("cargo:rustc-link-lib=dylib=stdc++");
}
