use ast::P;
use lexer::KeywordKind;
use lexer::SymbolKind;
use lexer::TokenKind::*;

use crate::dummy::Dummy;
use crate::*;

/// Parse local variable declaration.
pub fn parse_local_decl(ctx: &mut ParserContext, required: bool) -> Option<ast::Local> {
    let mut span = ctx.span();
    if ctx.token().kind != Keyword(KeywordKind::Let) {
        if required {
            parser_error!(span, "expected `let`");
        } else {
            return None;
        }
    }
    ctx.next();

    let mut is_const = true;
    if ctx.token().kind == Keyword(KeywordKind::Mut) {
        is_const = false;
        ctx.next();
    }

    let name = parse_identifier(ctx);

    let mut ty = None;
    if ctx.token().kind == Symbol(SymbolKind::Colon) {
        ctx.next();
        let res = parse_type(ctx);
        span = span.merge(&res.span);
        ty = Some(P(res));
    }

    let mut local_kind = ast::LocalKind::Decl;

    if ctx.token().kind == Symbol(SymbolKind::Equal) {
        ctx.next();
        let expr = parse_expr(ctx, true).unwrap();
        span = span.merge(&expr.span);
        local_kind = ast::LocalKind::Init { 0: expr };
    }

    let mut local = ast::Local::dummy();
    local.span = span;
    local.name = name;
    local.is_const = is_const;
    local.ty = ty;
    local.kind = local_kind;

    Some(local)
}

/// Parse block expression.
pub fn parse_if_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::If> {
    let span = ctx.span();
    if ctx.token().kind != Keyword(KeywordKind::If) {
        if required {
            parser_error!(span, "expected `if`");
        } else {
            return None;
        }
    }
    ctx.next();

    let cond = parse_expr(ctx, true).unwrap();
    let then = parse_block(ctx, true).unwrap();
    let mut else_block = None;

    if ctx.token().kind == Keyword(KeywordKind::Else) {
        ctx.next();
        let if_expr = parse_if_expr(ctx, false);
        if let Some(if_expr) = if_expr {
            else_block = Some(P(ast::ElseKind::If(if_expr)));
        } else {
            let block = parse_block(ctx, true).unwrap();
            else_block = Some(P(ast::ElseKind::Block(block)));
        }
    }

    let res = ast::If { span, cond, then, els: else_block };

    Some(res)
}

/// Parse expression.
pub fn parse_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let assign = parse_assign_expr(ctx, false);
    if let Some(assign) = assign {
        return Some(assign);
    }

    if required {
        parser_error!(ctx.span(), "expected expression");
    } else {
        return None;
    }
}
