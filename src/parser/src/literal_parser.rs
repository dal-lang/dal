use lexer::get_token_string;
use lexer::LiteralKind;
use lexer::TokenKind::*;

use crate::dummy::Dummy;
use crate::*;

/// Parse string literal.
pub fn parse_string_literal(ctx: &mut ParserContext) -> ast::StrLit {
    let span = ctx.span();

    if ctx.token().kind != Literal(LiteralKind::String) {
        parser_error!(span, "expected string literal");
    }

    let mut i = span.start_pos;
    let mut escaped = false;
    let mut first = true;
    let mut result = String::new();

    // loop through the string literal
    loop {
        if i >= span.end_pos - 1 {
            break;
        }

        let c = ctx.source.chars().nth(i).unwrap();
        if first {
            first = false;
        } else {
            if escaped {
                match c {
                    '\\' => result.push('\\'),
                    'r' => result.push('\r'),
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    '"' => result.push('"'),
                    _ => {}
                }
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                result.push(c);
            }
        }
        i += 1;
    }
    ctx.next();

    ast::StrLit { span, value: result }
}

/// Parse identifier.
pub fn parse_identifier(ctx: &mut ParserContext) -> ast::Ident {
    let span = ctx.span();

    if ctx.token().kind != Identifier {
        parser_error!(span, "expected identifier");
    }

    let mut res = ast::Ident::dummy();
    res.span = span;
    res.name = get_token_string(ctx.source, &ctx.token()).to_string();
    ctx.next();

    res
}

/// Parse int literal.
pub fn parse_int_literal(ctx: &mut ParserContext) -> ast::IntLit {
    let span = ctx.span();

    if ctx.token().kind != Literal(LiteralKind::Integer) {
        parser_error!(span, "expected integer literal");
    }

    let value = get_token_string(ctx.source, &ctx.token()).to_string();
    ctx.next();

    ast::IntLit { span, value }
}
