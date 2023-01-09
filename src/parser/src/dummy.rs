use ast::*;
use lexer::{Token, TokenKind};
use span::Span;

/// Trait that create new dummy object.
pub trait Dummy {
    /// Create a new dummy of this type.
    fn dummy() -> Self;
}

impl Dummy for Span {
    fn dummy() -> Self {
        Span { start_line: 0, start_pos: 0, end_line: 0, end_pos: 0 }
    }
}

impl Dummy for Token {
    fn dummy() -> Self {
        Token { kind: TokenKind::Eof, span: Span::dummy() }
    }
}

impl Dummy for Root {
    fn dummy() -> Self {
        Root { items: vec![] }
    }
}

impl Dummy for Ident {
    fn dummy() -> Self {
        Ident { span: Span::dummy(), name: String::new() }
    }
}

impl Dummy for Ty {
    fn dummy() -> Self {
        Ty { span: Span::dummy(), kind: TyKind::Prim(llvm::types::PrimitiveTy::Void) }
    }
}

impl Dummy for FnProto {
    fn dummy() -> Self {
        FnProto {
            span: Span::dummy(),
            name: Ident::dummy(),
            vis: Visibility::Private,
            params: vec![],
            ret_ty: P(Ty::dummy()),
        }
    }
}

impl Dummy for Block {
    fn dummy() -> Self {
        Block { span: Span::dummy(), stmts: vec![] }
    }
}

impl Dummy for FnDef {
    fn dummy() -> Self {
        FnDef { proto: FnProto::dummy(), body: Block::dummy(), span: Span::dummy() }
    }
}

impl Dummy for Local {
    fn dummy() -> Self {
        Local {
            span: Span::dummy(),
            name: Ident::dummy(),
            ty: None,
            kind: LocalKind::Decl,
            is_const: true,
        }
    }
}

impl Dummy for Expr {
    fn dummy() -> Self {
        Expr { kind: ExprKind::Ident(Ident::dummy()), span: Span::dummy() }
    }
}
