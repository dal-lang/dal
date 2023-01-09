use llvm::types::PrimitiveTy;
use span::Span;

use crate::ptr::P;

/// Ident holds information about an identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident {
    pub span: Span,
    pub name: String,
}

/// Root of the AST.
///
/// This is the root of the AST. It contains a list of top-level items.
#[derive(Clone, Debug)]
pub struct Root {
    pub items: Vec<P<Item>>,
}

/// An item in the AST.
///
/// This is a top-level item in the AST. It can be a function, extern block, or import and comment.
#[derive(Clone, Debug)]
pub enum Item {
    /// A function.
    Fn(FnDef),
    /// An extern block.
    Extern(ExternBlock),
    /// An import.
    Import(Span, StrLit),
    /// A comment.
    Comment(Span),
}

/// Complete definition of a function.
#[derive(Clone, Debug)]
pub struct FnDef {
    pub proto: FnProto,
    pub body: Block,
    pub span: Span,
}

/// Visibility of a function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    /// Private mean the function only accesable within it scope.
    Private,
    /// Public mean the function can be called from anywhere.
    Public,
}

/// Prototype of a function.
#[derive(Clone, Debug)]
pub struct FnProto {
    pub name: Ident,
    pub params: Vec<FnParam>,
    pub ret_ty: P<Ty>,
    pub span: Span,
    pub vis: Visibility,
}

/// A parameter of a function.
#[derive(Clone, Debug)]
pub struct FnParam {
    pub name: Ident,
    pub ty: P<Ty>,
    pub span: Span,
}

/// A block of statements.
#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<P<Stmt>>,
    pub span: Span,
}

/// A statement in the AST.
#[derive(Clone, Debug)]
pub enum Stmt {
    /// Block statement.
    Block(Block),
    /// Local variable declaration.
    Local(Local),
    /// Expression statement.
    Expr(Expr),
    /// Return statement.
    Return { expr: Option<Expr>, span: Span },
    /// If statement.
    If(If),
}

/// If statement.
#[derive(Clone, Debug)]
pub struct If {
    pub cond: Expr,
    pub then: Block,
    pub els: Option<P<ElseKind>>,
    pub span: Span,
}

/// Else kind.
#[derive(Clone, Debug)]
pub enum ElseKind {
    /// Else block.
    Block(Block),
    /// Else if statement.
    If(If),
}

/// A local variable declaration.
#[derive(Clone, Debug)]
pub struct Local {
    pub name: Ident,
    pub ty: Option<P<Ty>>,
    pub kind: LocalKind,
    pub span: Span,
    pub is_const: bool,
}

/// A local variable declaration kind.
#[derive(Clone, Debug)]
pub enum LocalKind {
    /// Declaration.
    Decl,
    /// Declaration with initialization.
    Init(Expr),
}

/// Extern block.
#[derive(Clone, Debug)]
pub struct ExternBlock {
    pub items: Vec<FnProto>,
    pub span: Span,
}

/// String literal.
#[derive(Clone, Debug)]
pub struct StrLit {
    pub value: String,
    pub span: Span,
}

/// Boolean literal.
#[derive(Clone, Debug)]
pub struct BoolLit {
    pub value: bool,
    pub span: Span,
}

/// An expression in the AST.
#[derive(Clone, Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

/// An expression kind.
#[derive(Clone, Debug)]
pub enum ExprKind {
    /// Literal expression.
    Lit(Lit),
    /// Binary expression.
    Binary(BinOp, P<Expr>, P<Expr>),
    /// Unary expression.
    Unary(UnOp, P<Expr>),
    /// Assignment expression.
    Assign(P<Expr>, P<Expr>),
    /// Call expression.
    Call(P<Expr>, Vec<P<Expr>>),
    /// Identifier expression.
    Ident(Ident),
    /// Cast expression.
    Cast(P<Expr>, P<Ty>),
}

/// A literal in the AST.
#[derive(Clone, Debug)]
pub enum Lit {
    /// Integer literal.
    Int(IntLit),
    /// String literal.
    Str(StrLit),
    /// Boolean literal.
    Bool(BoolLit),
}

/// An integer literal.
#[derive(Clone, Debug)]
pub struct IntLit {
    pub value: String,
    pub span: Span,
}

/// A type in the AST.
#[derive(Clone, Debug)]
pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}

/// A type kind.
#[derive(Clone, Debug)]
pub enum TyKind {
    /// Primitive type.
    Prim(PrimitiveTy),
    /// Pointer type.
    Ptr { parent: P<Ty>, is_const: bool },
    /// Array type.
    Array { parent: P<Ty>, size: IntLit },
}

/// A binary operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Modulo.
    Mod,
    /// Bitwise AND.
    BitAnd,
    /// Bitwise OR.
    BitOr,
    /// Bitwise XOR.
    BitXor,
    /// Bitwise left shift.
    Shl,
    /// Bitwise right shift.
    Shr,
    /// Logical AND.
    And,
    /// Logical OR.
    Or,
    /// Equality.
    Eq,
    /// Inequality.
    Ne,
    /// Less than.
    Lt,
    /// Less than or equal to.
    Le,
    /// Greater than.
    Gt,
    /// Greater than or equal to.
    Ge,
}

/// A unary operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    /// Negation.
    Neg,
    /// Bitwise NOT.
    Not,
    /// Logical NOT.
    LNot,
}
