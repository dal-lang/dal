use ast::P;
use lexer::KeywordKind::*;
use lexer::SymbolKind;
use lexer::TokenKind::*;

use crate::dummy::Dummy;
use crate::*;

/// Parse function call parameters.
pub fn parse_fn_call_params(ctx: &mut ParserContext) -> Vec<P<ast::Expr>> {
    let span = ctx.span();
    let mut params = vec![];

    if ctx.token().kind != Symbol(SymbolKind::LeftParen) {
        parser_error!(span, "expected `(`");
    }
    ctx.next();

    if ctx.token().kind == Symbol(SymbolKind::RightParen) {
        ctx.next();
        return params;
    }

    loop {
        let expr = parse_expr(ctx, true).unwrap();
        params.push(P(expr));

        if ctx.token().kind == Symbol(SymbolKind::RightParen) {
            ctx.next();
            break;
        }

        if ctx.token().kind != Symbol(SymbolKind::Comma) {
            parser_error!(span, "expected `,`");
        }
        ctx.next();
    }

    params
}

/// Parse params declaration.
pub fn parse_param_decls(ctx: &mut ParserContext) -> Vec<ast::FnParam> {
    if ctx.token().kind != Symbol(SymbolKind::LeftParen) {
        parser_error!(ctx.span(), "expected `(`");
    }
    ctx.next();

    let mut params = vec![];

    if ctx.token().kind == Symbol(SymbolKind::RightParen) {
        ctx.next();
        return params;
    }

    loop {
        let name = parse_identifier(ctx);
        if ctx.token().kind != Symbol(SymbolKind::Colon) {
            parser_error!(ctx.span(), "expected ':' after param name.");
        }
        ctx.next();

        let ty = parse_type(ctx);
        let res = ast::FnParam { name, ty: P(ty), span: ctx.span() };
        params.push(res);

        if ctx.token().kind == Symbol(SymbolKind::RightParen) {
            ctx.next();
            break;
        }

        if ctx.token().kind != Symbol(SymbolKind::Comma) {
            parser_error!(ctx.span(), "expected ',' or ')' after param declaration.");
        }
    }

    params
}

/// Parse function declaration.
pub fn parse_fn_decl(ctx: &mut ParserContext, required: bool) -> Option<ast::FnProto> {
    let span = ctx.span();
    let vis;

    if ctx.token().kind == Keyword(Pub) {
        vis = ast::Visibility::Public;
        ctx.next();
        if ctx.token().kind != Keyword(Fn) {
            parser_error!(span, "expected `fn`");
        }
        ctx.next();
    } else if ctx.token().kind == Keyword(Fn) {
        vis = ast::Visibility::Private;
    } else if required {
        parser_error!(span, "expected `pub fn` or `fn`, got: {}", ctx.token().kind);
    } else {
        return None;
    }

    let name = parse_identifier(ctx);

    if ctx.token().kind != Symbol(SymbolKind::LeftParen) {
        parser_error!(ctx.span(), "expected '('");
    }

    let params = parse_param_decls(ctx);

    let mut ret_ty = ast::Ty::dummy();
    ret_ty.kind = ast::TyKind::Prim(llvm::types::PrimitiveTy::Void);
    ret_ty.span = span;

    if ctx.token().kind == Symbol(SymbolKind::Arrow) {
        ctx.next();
        ret_ty = parse_type(ctx);
    }

    let mut proto = ast::FnProto::dummy();
    proto.name = name;
    proto.vis = vis;
    proto.params = params;
    proto.span = span.merge(&ret_ty.span);
    proto.ret_ty = P(ret_ty);
    Some(proto)
}

/// Parse function definition.
pub fn parse_fn_def(ctx: &mut ParserContext, required: bool) -> Option<ast::FnDef> {
    let proto = parse_fn_decl(ctx, required);
    if proto.is_none() {
        return None;
    }

    let proto = proto.unwrap();
    let body = parse_block(ctx, true).unwrap();

    let mut def = ast::FnDef::dummy();
    def.span = proto.span.merge(&body.span);
    def.proto = proto;
    def.body = body;
    Some(def)
}
