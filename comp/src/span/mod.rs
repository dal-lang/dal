/// Source file information.
pub struct SourceFile {
    /// Name of the source file.
    pub name: String,
    /// Path of the source file.
    pub path: String,
    /// Content of the source file.
    pub content: String,
    /// Position of the lines in the source file.
    pub lines: Vec<usize>,
}

impl SourceFile {
    /// Creates a new source file from the given path.
    pub fn new(path: &str) -> SourceFile {
        let content = std::fs::read_to_string(path).unwrap_or_else(|err| {
            println!("Error reading file: {}", err);
            return "".to_string();
        });

        let mut lines = Vec::new();
        lines.push(0);

        for (i, c) in content.chars().enumerate() {
            if c == '\n' {
                lines.push(i + 1);
            }
        }

        let name = std::path::Path::new(path).file_name().unwrap().to_str().unwrap().to_string();

        SourceFile { name, path: path.to_string(), content, lines }
    }

    /// Returns corresponding span of the end of file.
    pub fn eof_span(&self) -> Span {
        let lo = self.content.len();
        Span::new(lo, lo)
    }
}

/// Span of the source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    /// Creates a new span.
    pub fn new(lo: usize, hi: usize) -> Span {
        Span { lo, hi }
    }
}
