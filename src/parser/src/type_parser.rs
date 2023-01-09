use ast::P;
use lexer::{get_token_string, KeywordKind, SymbolKind, TokenKind::*};

use crate::dummy::Dummy;
use crate::*;

/// Parse type declaration.
pub fn parse_type(ctx: &mut ParserContext) -> ast::Ty {
    let mut span = ctx.span();

    // it's can be a pointer type like `*const i32` or `*mut i32`
    if ctx.token().kind == Symbol(SymbolKind::Asterisk) {
        ctx.next();
        let const_or_mut = ctx.token();
        let mut is_const = false;
        if const_or_mut.kind == Keyword(KeywordKind::Const) {
            is_const = true;
            ctx.next();
        } else if const_or_mut.kind == Keyword(KeywordKind::Mut) {
            ctx.next();
        } else {
            let end_span = ctx.span();
            parser_error!(span.merge(&end_span), "expected `const` or `mut`");
        }

        let parent_ty = parse_type(ctx);
        span = span.merge(&parent_ty.span);
        let mut ty = ast::Ty::dummy();
        ty.span = span;
        ty.kind = ast::TyKind::Ptr { parent: P(parent_ty), is_const };

        return ty;
    } else if ctx.token().kind == Identifier {
        let name = get_token_string(ctx.source, &ctx.token());
        let prim: llvm::types::PrimitiveTy;
        match name {
            "void" => prim = llvm::types::PrimitiveTy::Void,
            "u8" => prim = llvm::types::PrimitiveTy::U8,
            "i32" => prim = llvm::types::PrimitiveTy::I32,
            "isize" => prim = llvm::types::PrimitiveTy::ISize,
            _ => {
                parser_error!(span, "unknown type `{}`", name);
            }
        }
        let mut ty = ast::Ty::dummy();
        ty.span = span;
        ty.kind = ast::TyKind::Prim(prim);
        ctx.next();

        return ty;
    } else if ctx.token().kind == Symbol(SymbolKind::LeftBracket) {
        ctx.next();
        let parent_ty = parse_type(ctx);
        if ctx.token().kind != Symbol(SymbolKind::Semi) {
            let end_span = ctx.span();
            parser_error!(span.merge(&end_span), "expected `;`");
        }
        ctx.next();
        let size = parse_int_literal(ctx);
        if ctx.token().kind != Symbol(SymbolKind::RightBrace) {
            let end_span = ctx.span();
            parser_error!(span.merge(&end_span), "expected `]`");
        }
        span = span.merge(&ctx.span());
        ctx.next();
        let mut ty = ast::Ty::dummy();
        ty.span = span;
        ty.kind = ast::TyKind::Array { parent: P(parent_ty), size };
        return ty;
    } else {
        parser_error!(span, "expected type");
    }
}
