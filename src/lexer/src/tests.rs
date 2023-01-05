use super::*;

use expect_test::{expect, Expect};

fn check_lexing(input: &str, expected: Expect) {
    let actual: String = tokenize(input).map(|t| format!("{:#?}", t)).collect();
    expected.assert_eq(&actual);
}

#[test]
fn smoke() {
    check_lexing(
        "// This is a comment\nfn main() { println(\"Hello, world!\") }",
        expect![[r#"
Token {
    kind: Comment,
    span: Span {
        start_line: 1,
        start_pos: 0,
        end_line: 1,
        end_pos: 20,
    },
}Token {
    kind: Whitespace,
    span: Span {
        start_line: 1,
        start_pos: 20,
        end_line: 2,
        end_pos: 21,
    },
}Token {
    kind: Keyword(
        Fn,
    ),
    span: Span {
        start_line: 2,
        start_pos: 21,
        end_line: 2,
        end_pos: 23,
    },
}Token {
    kind: Whitespace,
    span: Span {
        start_line: 2,
        start_pos: 23,
        end_line: 2,
        end_pos: 24,
    },
}Token {
    kind: Identifier,
    span: Span {
        start_line: 2,
        start_pos: 24,
        end_line: 2,
        end_pos: 28,
    },
}Token {
    kind: Symbol(
        LeftParen,
    ),
    span: Span {
        start_line: 2,
        start_pos: 28,
        end_line: 2,
        end_pos: 29,
    },
}Token {
    kind: Symbol(
        RightParen,
    ),
    span: Span {
        start_line: 2,
        start_pos: 29,
        end_line: 2,
        end_pos: 30,
    },
}Token {
    kind: Whitespace,
    span: Span {
        start_line: 2,
        start_pos: 30,
        end_line: 2,
        end_pos: 31,
    },
}Token {
    kind: Symbol(
        LeftBrace,
    ),
    span: Span {
        start_line: 2,
        start_pos: 31,
        end_line: 2,
        end_pos: 32,
    },
}Token {
    kind: Whitespace,
    span: Span {
        start_line: 2,
        start_pos: 32,
        end_line: 2,
        end_pos: 33,
    },
}Token {
    kind: Identifier,
    span: Span {
        start_line: 2,
        start_pos: 33,
        end_line: 2,
        end_pos: 40,
    },
}Token {
    kind: Symbol(
        LeftParen,
    ),
    span: Span {
        start_line: 2,
        start_pos: 40,
        end_line: 2,
        end_pos: 41,
    },
}Token {
    kind: Literal(
        String,
    ),
    span: Span {
        start_line: 2,
        start_pos: 41,
        end_line: 2,
        end_pos: 56,
    },
}Token {
    kind: Symbol(
        RightParen,
    ),
    span: Span {
        start_line: 2,
        start_pos: 56,
        end_line: 2,
        end_pos: 57,
    },
}Token {
    kind: Whitespace,
    span: Span {
        start_line: 2,
        start_pos: 57,
        end_line: 2,
        end_pos: 58,
    },
}Token {
    kind: Symbol(
        RightBrace,
    ),
    span: Span {
        start_line: 2,
        start_pos: 58,
        end_line: 2,
        end_pos: 59,
    },
}"#]]
    )
}

#[test]
fn valid_source_code() {
    let code = r#"fn main() {
    println("Hello, world!")
}"#;
    let mut tokens = tokenize(code);
    assert_eq!(get_token_string(code, &tokens.next().unwrap()), "fn");
    tokens.next(); // Whitespace
    tokens.next(); // Identifier "main"
    tokens.next(); // LeftParen
    tokens.next(); // RightParen
    tokens.next(); // Whitespace
    tokens.next(); // LeftBrace
    tokens.next(); // Whitespace
    assert_eq!(get_token_string(code, &tokens.next().unwrap()), "println");
    tokens.next(); // LeftParen
    assert_eq!(get_token_string(code, &tokens.next().unwrap()), r#""Hello, world!""#);
    tokens.next(); // RightParen
    tokens.next(); // Whitespace
    tokens.next(); // RightBrace
    assert_eq!(tokens.next(), None);
}