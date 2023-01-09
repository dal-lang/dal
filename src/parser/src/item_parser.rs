use ast::P;
use lexer::KeywordKind::*;
use lexer::SymbolKind;
use lexer::TokenKind::*;

use crate::*;

/// Parse root items.
pub fn parse_items(ctx: &mut ParserContext) -> Vec<P<ast::Item>> {
    let mut items = vec![];

    loop {
        let fn_def = parse_fn_def(ctx, false);
        if let Some(fn_def) = fn_def {
            items.push(P(ast::Item::Fn(fn_def)));
            continue;
        }

        let import = parse_import(ctx, false);
        if let Some(import) = import {
            items.push(P(import));
            continue;
        }

        let ext = parse_extern(ctx, false);
        if let Some(ext) = ext {
            items.push(P(ext));
            continue;
        }

        // TODO: parse comment
        if ctx.token().kind == Comment {
            ctx.next();
            continue;
        }

        break;
    }

    items
}

/// Parse import statement.
pub fn parse_import(ctx: &mut ParserContext, required: bool) -> Option<ast::Item> {
    let mut span = ctx.span();

    if ctx.token().kind != Keyword(Import) {
        if required {
            parser_error!(span, "expected `import`");
        } else {
            return None;
        }
    }
    ctx.next();

    let str_lit = parse_string_literal(ctx);
    span = span.merge(&str_lit.span);
    let result = ast::Item::Import(span, str_lit);

    Some(result)
}

/// Parse extern block.
pub fn parse_extern(ctx: &mut ParserContext, required: bool) -> Option<ast::Item> {
    let mut span = ctx.span();

    if ctx.token().kind != Keyword(Extern) {
        if required {
            parser_error!(span, "expected `extern`");
        } else {
            return None;
        }
    }
    ctx.next();

    if ctx.token().kind != Symbol(SymbolKind::LeftBrace) {
        parser_error!(span, "expected `{{`");
    }
    ctx.next();

    let mut items = vec![];

    // because extern only contains function declarations, we will loop
    // parse_fn_decl until we reach the closing brace.
    loop {
        if ctx.token().kind == Symbol(SymbolKind::RightBrace) {
            span = span.merge(&ctx.span());
            ctx.next();
            break;
        }

        let item = parse_fn_decl(ctx, true).unwrap();
        items.push(item);
    }

    let extern_block = ast::ExternBlock { span, items };
    let result = ast::Item::Extern(extern_block);

    Some(result)
}

/// Parse function.
pub fn parse_fn(ctx: &mut ParserContext, required: bool) -> Option<ast::Item> {
    let fn_def = parse_fn_def(ctx, required);
    if let Some(fn_def) = fn_def {
        return Some(ast::Item::Fn(fn_def));
    } else {
        return None;
    }
}
