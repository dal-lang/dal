use ast::P;
use lexer::KeywordKind;
use lexer::LiteralKind;
use lexer::SymbolKind;
use lexer::TokenKind::*;

use crate::dummy::Dummy;
use crate::*;

/// Parse group expr.
pub fn parse_group_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();

    if ctx.token().kind != Symbol(SymbolKind::LeftParen) {
        if required {
            parser_error!(span, "expected `(`");
        } else {
            return None;
        }
    }
    ctx.next();

    let expr = parse_expr(ctx, true);

    if ctx.token().kind != Symbol(SymbolKind::RightParen) {
        parser_error!(span, "expected `)`");
    }
    ctx.next();

    expr
}

/// Parse primary expression.
pub fn parse_primary_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    if ctx.token().kind == Literal(LiteralKind::Integer) {
        let lit = parse_int_literal(ctx);
        let mut expr = ast::Expr::dummy();
        expr.span = lit.span.clone();
        expr.kind = ast::ExprKind::Lit(ast::Lit::Int(lit));

        return Some(expr);
    } else if ctx.token().kind == Literal(LiteralKind::String) {
        let lit = parse_string_literal(ctx);
        let mut expr = ast::Expr::dummy();
        expr.span = lit.span.clone();
        expr.kind = ast::ExprKind::Lit(ast::Lit::Str(lit));

        return Some(expr);
    } else if ctx.token().kind == Keyword(KeywordKind::True) {
        let lit = ast::BoolLit { value: true, span: ctx.span() };
        ctx.next();
        let mut expr = ast::Expr::dummy();
        expr.span = lit.span.clone();
        expr.kind = ast::ExprKind::Lit(ast::Lit::Bool(lit));

        return Some(expr);
    } else if ctx.token().kind == Keyword(KeywordKind::False) {
        let lit = ast::BoolLit { value: false, span: ctx.span() };
        ctx.next();
        let mut expr = ast::Expr::dummy();
        expr.span = lit.span.clone();
        expr.kind = ast::ExprKind::Lit(ast::Lit::Bool(lit));

        return Some(expr);
    } else if ctx.token().kind == Identifier {
        let ident = parse_identifier(ctx);
        let mut expr = ast::Expr::dummy();
        expr.span = ident.span.clone();
        expr.kind = ast::ExprKind::Ident(ident);

        return Some(expr);
    }

    let group = parse_group_expr(ctx, false);
    if group.is_some() {
        return group;
    }

    if required {
        parser_error!(ctx.span(), "expected primary expression");
    } else {
        None
    }
}

/// Parse postfix expression.
pub fn parse_post_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let primary = parse_primary_expr(ctx, required);
    if primary.is_none() {
        return None;
    }
    let primary = primary.unwrap();

    if ctx.token().kind == Symbol(SymbolKind::LeftParen) {
        let params = parse_fn_call_params(ctx);
        let mut expr = ast::Expr::dummy();
        expr.span = span.merge(&ctx.span());
        expr.kind = ast::ExprKind::Call(P(primary), params);

        Some(expr)
    } else {
        Some(primary)
    }
}

/// Parse unary expression.
pub fn parse_unary_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let op = match ctx.token().kind {
        Symbol(SymbolKind::Bang) => Some(ast::UnOp::LNot),
        Symbol(SymbolKind::Minus) => Some(ast::UnOp::Neg),
        Symbol(SymbolKind::Tilde) => Some(ast::UnOp::Not),
        _ => None,
    };
    if op.is_none() {
        return parse_post_expr(ctx, required);
    }
    ctx.next();

    let expr = parse_post_expr(ctx, true).unwrap();

    let mut expr_node = ast::Expr::dummy();
    expr_node.span = span.merge(&expr.span);
    expr_node.kind = ast::ExprKind::Unary(op.unwrap(), P(expr));

    Some(expr_node)
}

/// Parse cast expression.
pub fn parse_cast_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_unary_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    if ctx.token().kind != Keyword(KeywordKind::As) {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_type(ctx);

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Cast(P(lhs), P(rhs));

    Some(expr)
}

/// Parse multiplication expression.
pub fn parse_mul_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_cast_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    let op = match ctx.token().kind {
        Symbol(SymbolKind::Asterisk) => Some(ast::BinOp::Mul),
        Symbol(SymbolKind::Slash) => Some(ast::BinOp::Div),
        Symbol(SymbolKind::Percent) => Some(ast::BinOp::Mod),
        _ => None,
    };
    if op.is_none() {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_cast_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Binary(op.unwrap(), P(lhs), P(rhs));

    Some(expr)
}

/// Parse addition expression.
pub fn parse_add_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_mul_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    let op = match ctx.token().kind {
        Symbol(SymbolKind::Plus) => Some(ast::BinOp::Add),
        Symbol(SymbolKind::Minus) => Some(ast::BinOp::Sub),
        _ => None,
    };
    if op.is_none() {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_mul_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Binary(op.unwrap(), P(lhs), P(rhs));

    Some(expr)
}

/// Parse bit shift expression.
pub fn parse_bit_shift_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_add_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    let op = match ctx.token().kind {
        Symbol(SymbolKind::LeftShift) => Some(ast::BinOp::Shl),
        Symbol(SymbolKind::RightShift) => Some(ast::BinOp::Shr),
        _ => None,
    };
    if op.is_none() {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_add_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Binary(op.unwrap(), P(lhs), P(rhs));

    Some(expr)
}

/// Parse bit and expression.
pub fn parse_bit_and_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_bit_shift_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    if ctx.token().kind != Symbol(SymbolKind::And) {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_bit_shift_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Binary(ast::BinOp::BitAnd, P(lhs), P(rhs));

    Some(expr)
}

/// Parse bit xor expression.
pub fn parse_bit_xor_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_bit_and_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    if ctx.token().kind != Symbol(SymbolKind::Caret) {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_bit_and_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Binary(ast::BinOp::BitXor, P(lhs), P(rhs));

    Some(expr)
}

/// Parse bit or expression.
pub fn parse_bit_or_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_bit_xor_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    if ctx.token().kind != Symbol(SymbolKind::Or) {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_bit_xor_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Binary(ast::BinOp::BitOr, P(lhs), P(rhs));

    Some(expr)
}

/// Parse comparison expressions.
pub fn parse_cmp_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_bit_or_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    let op = match ctx.token().kind {
        Symbol(SymbolKind::Less) => Some(ast::BinOp::Lt),
        Symbol(SymbolKind::LessEqual) => Some(ast::BinOp::Le),
        Symbol(SymbolKind::Greater) => Some(ast::BinOp::Gt),
        Symbol(SymbolKind::GreaterEqual) => Some(ast::BinOp::Ge),
        Symbol(SymbolKind::EqualEqual) => Some(ast::BinOp::Eq),
        Symbol(SymbolKind::BangEqual) => Some(ast::BinOp::Ne),
        _ => None,
    };
    if op.is_none() {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_bit_or_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Binary(op.unwrap(), P(lhs), P(rhs));

    Some(expr)
}

/// Parse bool and expression.
pub fn parse_bool_and_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_cmp_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    if ctx.token().kind != Symbol(SymbolKind::AndAnd) {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_cmp_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Binary(ast::BinOp::And, P(lhs), P(rhs));

    Some(expr)
}

/// Parse bool or expression.
pub fn parse_bool_or_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();
    let lhs = parse_bool_and_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    if ctx.token().kind != Symbol(SymbolKind::OrOr) {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_bool_and_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Binary(ast::BinOp::Or, P(lhs), P(rhs));

    Some(expr)
}

/// Parse assign expr.
pub fn parse_assign_expr(ctx: &mut ParserContext, required: bool) -> Option<ast::Expr> {
    let span = ctx.span();

    let lhs = parse_bool_or_expr(ctx, required);
    if lhs.is_none() {
        return None;
    }

    if ctx.token().kind != Symbol(SymbolKind::Equal) {
        return lhs;
    }
    ctx.next();

    let lhs = lhs.unwrap();

    let rhs = parse_bool_or_expr(ctx, true).unwrap();

    let mut expr = ast::Expr::dummy();
    expr.span = span.merge(&rhs.span);
    expr.kind = ast::ExprKind::Assign(P(lhs), P(rhs));

    Some(expr)
}
