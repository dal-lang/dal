//! Simple CLI parser for dal compiler.

use std::{cmp, collections::HashMap, env, process::exit};

/// App is the main struct for CLI parser.
/// It contains all commands and arguments.
pub struct App {
    /// Name of the app
    name: String,
    /// Description of the app
    description: String,
    /// Version of the app
    version: String,
    /// Commands of the app
    commands: HashMap<String, Command>,
    /// Longest command name, used for formatting
    /// help message
    longest_command: usize,
}

impl App {
    /// Create a new App by the given name, description, and version.
    pub fn new(name: &str, description: &str, version: &str) -> App {
        App {
            name: name.to_string(),
            description: description.to_string(),
            version: version.to_string(),
            commands: HashMap::new(),
            longest_command: "version".len(),
        }
    }

    /// Error handler for unknown command.
    fn err_unknown_command(&self, command_name: &str) -> ! {
        println!("error: Unknown command `{}`", command_name);
        self.show_help(1)
    }

    /// Show app version and exit.
    fn show_version(&self) -> ! {
        println!("{} version: {}", self.name, self.version);
        exit(0)
    }

    /// Show app help message and exit.
    fn show_help(&self, code: i32) -> ! {
        println!("{} - {}", self.name, self.description);
        println!("Usage: {} <command> [options]", self.name);
        if !self.commands.is_empty() {
            println!("\nCommands:");
            for (k, v) in &self.commands {
                println!(
                    "   {}  {}{}",
                    k,
                    " ".repeat(self.longest_command - k.len()),
                    v.description
                );
            }

            println!("   help  {}Show this message", " ".repeat(self.longest_command - 4));
            println!("   version  {}Show app version", " ".repeat(self.longest_command - 7));
        }
        exit(code)
    }

    /// Show specific command help message and exit.
    fn show_command_help(&self, command_name: &str) -> ! {
        if let Some(cmd) = self.commands.get(command_name) {
            cmd.show_help(0)
        }

        if command_name == "help" {
            println!("help - Show command usage");
            println!("\nUsage: {} help <command>", self.name);
            exit(0)
        }

        if command_name == "version" {
            println!("version - Show app version");
            println!("\nusage: {} version", self.name);
            exit(0)
        }

        self.err_unknown_command(command_name)
    }

    /// Parse will parse available arguments from `env::args()` and
    /// run the corresponding command.
    pub fn parse(&mut self) -> ! {
        let args = env::args().collect::<Vec<String>>();
        let lists = args[1..].to_vec();

        if lists.is_empty() {
            self.show_help(1)
        }

        if ["version", "-V", "--version"].contains(&lists[0].as_str()) {
            self.show_version()
        }

        if ["help", "-h", "--help"].contains(&lists[0].as_str()) {
            if lists.len() > 1 {
                self.show_command_help(&lists[1])
            }
            self.show_help(0)
        }

        if let Some(cmd) = self.commands.get(&lists[0]) {
            cmd.run(&lists[1..])
        } else {
            self.err_unknown_command(&lists[0])
        }
    }

    /// Add a command to the app.
    pub fn add_command(&mut self, command: Command) {
        self.longest_command = cmp::max(self.longest_command, command.name.len());
        self.commands.insert(command.name.clone(), command.prog(&self.name));
    }
}

/// Command is a struct for a command.
pub struct Command {
    /// Name of the app.
    prog: String,
    /// Name of the command.
    name: String,
    /// Description of the command.
    description: String,
    /// Usage of the command.
    usage: String,
    /// Arguments of the command.
    args: HashMap<String, Arg>,
    /// It is used to store short name of arguments so that we can
    /// find the argument by its short name.
    shorts: HashMap<String, String>,
    /// Handler of the command.
    handler: Option<Box<dyn Fn(&CliContext) -> i32>>,
}

impl Command {
    /// Create a new command.
    pub fn new() -> Command {
        Command {
            prog: String::new(),
            name: String::new(),
            description: String::new(),
            usage: String::new(),
            args: HashMap::new(),
            shorts: HashMap::new(),
            handler: None,
        }
    }

    /// Set the command name.
    pub fn name(mut self, name: &str) -> Command {
        self.name = name.to_string();
        self
    }

    /// Set the command description.
    pub fn desc(mut self, desc: &str) -> Command {
        self.description = desc.to_string();
        self
    }

    /// Set the command usage.
    pub fn usage(mut self, usage: &str) -> Command {
        self.usage = usage.to_string();
        self
    }

    /// Add an argument to the command.
    pub fn arg<F>(mut self, f: F) -> Command
    where
        F: FnOnce(Arg) -> Arg,
    {
        let arg = f(Arg::new());
        if arg.short.is_some() {
            self.shorts.insert(arg.short.as_ref().unwrap().to_string(), arg.name.clone());
        }
        self.args.insert(arg.name.clone(), arg);
        self
    }

    /// Set the program name.
    fn prog(mut self, prog: &str) -> Command {
        self.prog = prog.to_string();
        self
    }

    /// Set the command handler.
    pub fn handler<F>(mut self, f: F) -> Command
    where
        F: Fn(&CliContext) -> i32 + 'static,
    {
        self.handler = Some(Box::new(f));
        self
    }

    /// Add the command to the app.
    pub fn build(self, app: &mut App) {
        app.add_command(self);
    }

    /// Show the command help message and exit.
    fn show_help(&self, code: i32) -> ! {
        println!("{} - {}", self.name, self.description);
        println!("Usage: {} {}", self.prog, self.usage);

        if !self.args.is_empty() {
            println!("\nOptions:");

            let mut details: Vec<(String, String, String)> =
                vec![("--help, -h".to_string(), "".to_string(), "Show this message".to_string())];

            let mut longest_opt = details[0].0.len();
            let mut longest_hint = details[0].1.len();

            for (_, arg) in &self.args {
                let mut opt = format!("--{}", arg.name);
                if arg.short.is_some() {
                    opt.push_str(", -");
                    opt.push_str(arg.short.as_ref().unwrap());
                }

                longest_opt = cmp::max(longest_opt, opt.len());
                longest_hint = cmp::max(longest_hint, arg.hint.len());
                details.push((opt, arg.hint.clone(), arg.description.clone()));
            }

            for arg in details {
                println!(
                    "  {}  {}{}  {}{}",
                    arg.0,
                    " ".repeat(longest_opt - arg.0.len()),
                    arg.1,
                    " ".repeat(longest_hint - arg.1.len()),
                    arg.2
                );
            }
        }

        exit(code)
    }

    /// Parse the command arguments and run the handler.
    fn run(&self, lists: &[String]) -> ! {
        if self.handler.is_none() {
            println!("error: no handler for command '{}'", self.name);
            exit(1)
        }

        let mut ctx = CliContext::new();
        let mut i = 0;

        while i < lists.len() {
            match &lists[i] {
                s if s == "--help" || s == "-h" => self.show_help(0),
                s if s.starts_with("-") => {
                    let (name, is_long) = if s.starts_with("--") {
                        (s.trim_start_matches("--"), true)
                    } else {
                        (s.trim_start_matches("-"), false)
                    };

                    let arg = if is_long {
                        self.args.get(name)
                    } else {
                        self.shorts.get(name).and_then(|k| self.args.get(k))
                    };

                    if let Some(arg) = arg {
                        match arg.kind {
                            ArgKind::Bool => {
                                ctx.insert_bool(arg);
                            }
                            ArgKind::String => {
                                let val = lists.get(i + 1).unwrap_or_else(|| {
                                    println!("error: missing value for argument '{}'", arg.name);
                                    exit(1)
                                });
                                ctx.insert_string(arg, val);
                                i += 1;
                            }
                        }
                    } else {
                        println!("error: unknown argument '{}'", name);
                        exit(1)
                    }
                }
                _ => {
                    ctx.frees.push(lists[i].clone());
                }
            }
            i += 1;
        }

        exit((self.handler.as_ref().unwrap())(&ctx))
    }
}

/// The command argument.
pub struct Arg {
    /// The argument name e.g. "verbose" ("--verbose").
    name: String,
    /// The argument short name e.g. "v" ("-v").
    short: Option<String>,
    /// The argument description.
    description: String,
    /// The argument hint. used for help message.
    hint: String,
    /// The argument kind.
    kind: ArgKind,
}

impl Arg {
    /// Create a new argument.
    pub fn new() -> Arg {
        Arg {
            name: String::new(),
            short: None,
            description: String::new(),
            hint: String::new(),
            kind: ArgKind::Bool,
        }
    }

    /// Set the argument name.
    pub fn name(mut self, name: &str) -> Arg {
        self.name = name.to_string();
        self
    }

    /// Set the argument short name.
    pub fn short(mut self, short: &str) -> Arg {
        self.short = match short {
            "" => None,
            _ => Some(short.to_string()),
        };
        self
    }

    /// Set the argument description.
    pub fn desc(mut self, desc: &str) -> Arg {
        self.description = desc.to_string();
        self
    }

    /// Set the argument hint.
    pub fn hint(mut self, hint: &str) -> Arg {
        self.hint = hint.to_string();
        self
    }

    /// Set the argument kind.
    pub fn kind(mut self, kind: ArgKind) -> Arg {
        self.kind = kind;
        self
    }
}

/// The argument kind.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArgKind {
    Bool,
    String,
}

/// CliContext is the context for the command handler.
/// It contains the parsed arguments.
pub struct CliContext {
    /// The short keys used for finding the argument index.
    short_keys: HashMap<String, usize>,
    /// The long keys used for finding the argument index.
    long_keys: HashMap<String, usize>,
    /// The string arguments.
    string_args: Vec<String>,
    /// The free arguments.
    frees: Vec<String>,
}

impl CliContext {
    /// Create a new CliContext.
    fn new() -> CliContext {
        CliContext {
            short_keys: HashMap::new(),
            long_keys: HashMap::new(),
            string_args: vec![String::new()], // index 0 reserved for boolean arguments.
            frees: Vec::new(),
        }
    }

    /// Insert a boolean argument.
    fn insert_bool(&mut self, arg: &Arg) {
        self.long_keys.insert(arg.name.to_string(), 0);
        if arg.short.is_some() {
            self.short_keys.insert(arg.short.as_ref().unwrap().to_string(), 0);
        }
    }

    /// Insert a string argument.
    fn insert_string(&mut self, arg: &Arg, val: &str) {
        self.long_keys.insert(arg.name.to_string(), self.string_args.len());
        if arg.short.is_some() {
            self.short_keys.insert(arg.short.as_ref().unwrap().to_string(), self.string_args.len());
        }
        self.string_args.push(val.to_string());
    }

    /// Get the boolean argument value.
    pub fn get_bool(&self, name: &str) -> bool {
        self.long_keys.contains_key(name) || self.short_keys.contains_key(name)
    }

    /// Get the string argument value.
    pub fn get_string(&self, name: &str) -> Option<&str> {
        let index = self.long_keys.get(name).or_else(|| self.short_keys.get(name));
        if let Some(index) = index {
            // Because index 0 is reserved for boolean arguments, so if the index is 0, it means
            // the argument is a boolean argument.
            if *index == 0 { None } else { Some(&self.string_args[*index]) }
        } else {
            None
        }
    }

    /// Get the free arguments.
    pub fn get_frees(&self) -> &[String] {
        &self.frees
    }
}
