use super::*;

use expect_test::{expect, Expect};

fn test_parse(input: &str, expected: Expect) {
    let mut parser = new_filtered_parser(input, FilterOption::Whitespace);
    let root = parse_root(&mut parser);
    expected.assert_eq(&format!("{:#?}", root));
}

#[test]
fn import() {
    test_parse(
        r#"import "std""#,
        expect![[r#"
Root {
    items: [
        Import(
            Span {
                start_line: 1,
                start_pos: 0,
                end_line: 1,
                end_pos: 12,
            },
            StrLit {
                value: "std",
                span: Span {
                    start_line: 1,
                    start_pos: 7,
                    end_line: 1,
                    end_pos: 12,
                },
            },
        ),
    ],
}"#]],
    );

    let mut parser = new_filtered_parser(r#"import "std""#, FilterOption::Whitespace);
    let ast = parse_root(&mut parser);
    let root = ast.items;
    let root = root[0].clone().into_inner();
    match root {
        ast::Item::Import(_, str) => {
            assert_eq!(str.value, "std");
        }
        _ => panic!("expected import"),
    }
}

#[test]
fn shadow() {
    test_parse(
        r#"import "std"

extern {
    pub fn x(t: *const u8) -> u8
}

pub fn main() {
    let x = 10
    return
}"#,
        expect![[r#"
Root {
    items: [
        Import(
            Span {
                start_line: 1,
                start_pos: 0,
                end_line: 1,
                end_pos: 12,
            },
            StrLit {
                value: "std",
                span: Span {
                    start_line: 1,
                    start_pos: 7,
                    end_line: 1,
                    end_pos: 12,
                },
            },
        ),
        Extern(
            ExternBlock {
                items: [
                    FnProto {
                        name: Ident {
                            span: Span {
                                start_line: 4,
                                start_pos: 34,
                                end_line: 4,
                                end_pos: 35,
                            },
                            name: "x",
                        },
                        params: [
                            FnParam {
                                name: Ident {
                                    span: Span {
                                        start_line: 4,
                                        start_pos: 36,
                                        end_line: 4,
                                        end_pos: 37,
                                    },
                                    name: "t",
                                },
                                ty: Ty {
                                    kind: Ptr {
                                        parent: Ty {
                                            kind: Prim(
                                                U8,
                                            ),
                                            span: Span {
                                                start_line: 4,
                                                start_pos: 46,
                                                end_line: 4,
                                                end_pos: 48,
                                            },
                                        },
                                        is_const: true,
                                    },
                                    span: Span {
                                        start_line: 4,
                                        start_pos: 39,
                                        end_line: 4,
                                        end_pos: 48,
                                    },
                                },
                                span: Span {
                                    start_line: 4,
                                    start_pos: 48,
                                    end_line: 4,
                                    end_pos: 49,
                                },
                            },
                        ],
                        ret_ty: Ty {
                            kind: Prim(
                                U8,
                            ),
                            span: Span {
                                start_line: 4,
                                start_pos: 53,
                                end_line: 4,
                                end_pos: 55,
                            },
                        },
                        span: Span {
                            start_line: 4,
                            start_pos: 27,
                            end_line: 4,
                            end_pos: 55,
                        },
                        vis: Public,
                    },
                ],
                span: Span {
                    start_line: 3,
                    start_pos: 14,
                    end_line: 5,
                    end_pos: 57,
                },
            },
        ),
        Fn(
            FnDef {
                proto: FnProto {
                    name: Ident {
                        span: Span {
                            start_line: 7,
                            start_pos: 66,
                            end_line: 7,
                            end_pos: 70,
                        },
                        name: "main",
                    },
                    params: [],
                    ret_ty: Ty {
                        kind: Prim(
                            Void,
                        ),
                        span: Span {
                            start_line: 7,
                            start_pos: 59,
                            end_line: 7,
                            end_pos: 62,
                        },
                    },
                    span: Span {
                        start_line: 7,
                        start_pos: 59,
                        end_line: 7,
                        end_pos: 62,
                    },
                    vis: Public,
                },
                body: Block {
                    stmts: [
                        Local(
                            Local {
                                name: Ident {
                                    span: Span {
                                        start_line: 8,
                                        start_pos: 83,
                                        end_line: 8,
                                        end_pos: 84,
                                    },
                                    name: "x",
                                },
                                ty: None,
                                kind: Init(
                                    Expr {
                                        kind: Lit(
                                            Int(
                                                IntLit {
                                                    value: "10",
                                                    span: Span {
                                                        start_line: 8,
                                                        start_pos: 87,
                                                        end_line: 8,
                                                        end_pos: 89,
                                                    },
                                                },
                                            ),
                                        ),
                                        span: Span {
                                            start_line: 8,
                                            start_pos: 87,
                                            end_line: 8,
                                            end_pos: 89,
                                        },
                                    },
                                ),
                                span: Span {
                                    start_line: 8,
                                    start_pos: 79,
                                    end_line: 8,
                                    end_pos: 89,
                                },
                                is_const: true,
                            },
                        ),
                        Return {
                            expr: None,
                            span: Span {
                                start_line: 9,
                                start_pos: 94,
                                end_line: 9,
                                end_pos: 100,
                            },
                        },
                    ],
                    span: Span {
                        start_line: 7,
                        start_pos: 73,
                        end_line: 10,
                        end_pos: 102,
                    },
                },
                span: Span {
                    start_line: 7,
                    start_pos: 59,
                    end_line: 10,
                    end_pos: 102,
                },
            },
        ),
    ],
}"#]],
    )
}
