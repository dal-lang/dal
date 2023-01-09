use ast::P;
use lexer::KeywordKind;
use lexer::SymbolKind;
use lexer::TokenKind::*;

use crate::dummy::Dummy;
use crate::*;

/// BlockExpr holds result from If and Block itself.
#[derive(Debug, Clone)]
pub enum BlockExpr {
    If(ast::If),
    Block(ast::Block),
}

/// NonBlockExpr holds result from Return and Expr.
#[derive(Debug, Clone)]
pub enum NonBlockExpr {
    Return(ast::Stmt),
    Expr(ast::Expr),
}

/// Parse block statement.
pub fn parse_block(ctx: &mut ParserContext, required: bool) -> Option<ast::Block> {
    let mut span = ctx.span();

    if ctx.token().kind != Symbol(SymbolKind::LeftBrace) {
        if required {
            parser_error!(span, "expected `{{`");
        } else {
            return None;
        }
    }
    ctx.next();

    let mut stmts = vec![];

    loop {
        let mut stmt: Option<ast::Stmt> = None;
        let local = parse_local_decl(ctx, false);
        if let Some(local) = local {
            stmt = Some(ast::Stmt::Local(local));
        }

        if stmt.is_none() {
            let block_expr = parse_block_expr(ctx, false);
            if let Some(block_expr) = block_expr {
                match block_expr {
                    BlockExpr::If(if_expr) => {
                        stmt = Some(ast::Stmt::If(if_expr));
                    }
                    BlockExpr::Block(block) => {
                        stmt = Some(ast::Stmt::Block(block));
                    }
                }
            }
        }

        if stmt.is_none() {
            let non_block_expr = parse_non_block_expr(ctx, false);
            if let Some(non_block_expr) = non_block_expr {
                match non_block_expr {
                    NonBlockExpr::Return(ret) => {
                        let ret = ret;
                        stmt = Some(ret);
                    }
                    NonBlockExpr::Expr(expr) => {
                        let expr = expr;
                        stmt = Some(ast::Stmt::Expr(expr));
                    }
                }
            }
        }

        if let Some(stmt) = stmt {
            stmts.push(P(stmt));
        }

        if ctx.token().kind == Symbol(SymbolKind::RightBrace) {
            span = span.merge(&ctx.span());
            ctx.next();
            break;
        }
    }

    let mut block = ast::Block::dummy();
    block.span = span;
    block.stmts = stmts;
    Some(block)
}

/// Parse block expression.
pub fn parse_block_expr(ctx: &mut ParserContext, required: bool) -> Option<BlockExpr> {
    let span = ctx.span();
    let if_expr = parse_if_expr(ctx, false);
    if let Some(if_expr) = if_expr {
        return Some(BlockExpr::If(if_expr));
    }

    let block = parse_block(ctx, false);
    if let Some(block) = block {
        return Some(BlockExpr::Block(block));
    }

    if required {
        parser_error!(span, "expected `if` or `{{`");
    } else {
        None
    }
}

/// Parse non-block expression.
pub fn parse_non_block_expr(ctx: &mut ParserContext, required: bool) -> Option<NonBlockExpr> {
    let span = ctx.span();
    let ret_stmt = parse_return(ctx, false);
    if let Some(ret_stmt) = ret_stmt {
        return Some(NonBlockExpr::Return(ret_stmt));
    }

    let assign_expr = parse_assign_expr(ctx, false);
    if let Some(assign_expr) = assign_expr {
        return Some(NonBlockExpr::Expr(assign_expr));
    }

    if required {
        parser_error!(span, "expected `return` or `=`");
    } else {
        None
    }
}

/// Parse return statement.
pub fn parse_return(ctx: &mut ParserContext, required: bool) -> Option<ast::Stmt> {
    let mut span = ctx.span();
    if ctx.token().kind != Keyword(KeywordKind::Return) {
        if required {
            parser_error!(span, "expected `return`");
        } else {
            return None;
        }
    }
    ctx.next();

    let mut expr = None;
    if let Some(e) = parse_expr(ctx, false) {
        span = span.merge(&e.span);
        expr = Some(e);
    }

    return Some(ast::Stmt::Return { expr, span });
}
