// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use std::{
    cell::{Ref, RefCell},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Root,
    FnProto,
    FnDef,
    FnDecl,
    ParamDecl,
    Type,
    Block,
    ExternBlock,
    Import,
    Return,
    VarDecl,
    BinOpExpr,
    UnaryOpExpr,
    CallExpr,
    Ident,
    ArrayAccessExpr,
    CastExpr,
    Void,
    IfExpr,
    AsmExpr,
    StrLit,
    NumLit,
    BoolLit,
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Root => write!(f, "Root"),
            NodeKind::FnProto => write!(f, "FnProto"),
            NodeKind::FnDef => write!(f, "FnDef"),
            NodeKind::FnDecl => write!(f, "FnDecl"),
            NodeKind::ParamDecl => write!(f, "ParamDecl"),
            NodeKind::Type => write!(f, "Type"),
            NodeKind::Block => write!(f, "Block"),
            NodeKind::ExternBlock => write!(f, "ExternBlock"),
            NodeKind::Import => write!(f, "Import"),
            NodeKind::Return => write!(f, "Return"),
            NodeKind::VarDecl => write!(f, "VarDecl"),
            NodeKind::BinOpExpr => write!(f, "BinOpExpr"),
            NodeKind::UnaryOpExpr => write!(f, "UnaryOpExpr"),
            NodeKind::CallExpr => write!(f, "CallExpr"),
            NodeKind::Ident => write!(f, "Ident"),
            NodeKind::ArrayAccessExpr => write!(f, "ArrayAccessExpr"),
            NodeKind::CastExpr => write!(f, "CastExpr"),
            NodeKind::Void => write!(f, "Void"),
            NodeKind::IfExpr => write!(f, "IfExpr"),
            NodeKind::AsmExpr => write!(f, "AsmExpr"),
            NodeKind::StrLit => write!(f, "StrLit"),
            NodeKind::NumLit => write!(f, "NumLit"),
            NodeKind::BoolLit => write!(f, "BoolLit"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootNode {
    pub children: RefCell<Vec<RefCell<Node>>>,
}

impl RootNode {
    pub fn new() -> Self {
        Self {
            children: RefCell::new(vec![]),
        }
    }

    pub fn push(&self, node: Node) {
        self.children.borrow_mut().push(RefCell::new(node));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FnProtoVisibMod {
    Public,
    Private,
}

impl Display for FnProtoVisibMod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FnProtoVisibMod::Public => write!(f, "Public"),
            FnProtoVisibMod::Private => write!(f, "Private"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnProtoNode {
    pub visib_mod: RefCell<FnProtoVisibMod>,
    pub name: RefCell<String>,
    pub params: RefCell<Vec<RefCell<Node>>>,
    pub ret_type: RefCell<Node>,
}

impl FnProtoNode {
    pub fn new() -> Self {
        Self {
            visib_mod: RefCell::new(FnProtoVisibMod::Private),
            name: RefCell::new(String::new()),
            params: RefCell::new(vec![]),
            ret_type: RefCell::new(Node::new(NodeKind::Void)),
        }
    }

    pub fn push_param(&self, param: Node) {
        self.params.borrow_mut().push(RefCell::new(param));
    }

    pub fn set_ret_type(&self, ret_type: Node) {
        self.ret_type.replace(ret_type);
    }

    pub fn set_name(&self, name: String) {
        self.name.replace(name);
    }

    pub fn set_visib_mod(&self, visib_mod: FnProtoVisibMod) {
        self.visib_mod.replace(visib_mod);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnDefNode {
    pub proto: RefCell<Node>,
    pub body: RefCell<Node>,
}

impl FnDefNode {
    pub fn new() -> Self {
        Self {
            proto: RefCell::new(Node::new(NodeKind::FnProto)),
            body: RefCell::new(Node::new(NodeKind::Block)),
        }
    }

    pub fn set_proto(&self, proto: Node) {
        self.proto.replace(proto);
    }

    pub fn set_body(&self, body: Node) {
        self.body.replace(body);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnDeclNode {
    pub proto: RefCell<Node>,
}

impl FnDeclNode {
    pub fn new() -> Self {
        Self {
            proto: RefCell::new(Node::new(NodeKind::FnProto)),
        }
    }

    pub fn set_proto(&self, proto: Node) {
        self.proto.replace(proto);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamDeclNode {
    pub name: RefCell<String>,
    pub ty: RefCell<Node>,
}

impl ParamDeclNode {
    pub fn new() -> Self {
        Self {
            name: RefCell::new(String::new()),
            ty: RefCell::new(Node::new(NodeKind::Type)),
        }
    }

    pub fn set_name(&self, name: String) {
        self.name.replace(name);
    }

    pub fn set_ty(&self, ty: Node) {
        self.ty.replace(ty);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeNodeKind {
    Primitive,
    Array,
    Pointer,
}

impl Display for TypeNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeNodeKind::Primitive => write!(f, "Primitive"),
            TypeNodeKind::Array => write!(f, "Array"),
            TypeNodeKind::Pointer => write!(f, "Pointer"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeNode {
    pub kind: RefCell<TypeNodeKind>,
    pub name: RefCell<String>,
    pub ty: RefCell<Option<Node>>,
    pub size: RefCell<Option<Node>>,
    pub is_const: RefCell<bool>,
}

impl TypeNode {
    pub fn new() -> Self {
        Self {
            kind: RefCell::new(TypeNodeKind::Primitive),
            name: RefCell::new(String::new()),
            ty: RefCell::new(None),
            size: RefCell::new(None),
            is_const: RefCell::new(false),
        }
    }

    pub fn set_kind(&self, kind: TypeNodeKind) {
        self.kind.replace(kind);
    }

    pub fn set_name(&self, name: String) {
        self.name.replace(name);
    }

    pub fn set_ty(&self, ty: Node) {
        self.ty.replace(Some(ty));
    }

    pub fn set_size(&self, size: Node) {
        self.size.replace(Some(size));
    }

    pub fn set_is_const(&self, is_const: bool) {
        self.is_const.replace(is_const);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockNode {
    pub children: RefCell<Vec<RefCell<Node>>>,
}

impl BlockNode {
    pub fn new() -> Self {
        Self {
            children: RefCell::new(vec![]),
        }
    }

    pub fn push(&self, node: Node) {
        self.children.borrow_mut().push(RefCell::new(node));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternNode {
    pub fn_decls: RefCell<Vec<RefCell<Node>>>,
}

impl ExternNode {
    pub fn new() -> Self {
        Self {
            fn_decls: RefCell::new(vec![]),
        }
    }

    pub fn push(&self, node: Node) {
        self.fn_decls.borrow_mut().push(RefCell::new(node));
    }

    pub fn set_fn_decls(&self, fn_decls: Vec<RefCell<Node>>) {
        self.fn_decls.replace(fn_decls);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportNode {
    pub path: RefCell<String>,
}

impl ImportNode {
    pub fn new() -> Self {
        Self {
            path: RefCell::new(String::new()),
        }
    }

    pub fn set_path(&self, path: String) {
        self.path.replace(path);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnNode {
    pub expr: RefCell<Option<Node>>,
}

impl ReturnNode {
    pub fn new() -> Self {
        Self {
            expr: RefCell::new(None),
        }
    }

    pub fn set_expr(&self, expr: Node) {
        self.expr.replace(Some(expr));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDeclNode {
    pub name: RefCell<String>,
    pub ty: RefCell<Option<Node>>,
    pub expr: RefCell<Option<Node>>,
    pub is_const: RefCell<bool>,
}

impl VarDeclNode {
    pub fn new() -> Self {
        Self {
            name: RefCell::new(String::new()),
            ty: RefCell::new(None),
            expr: RefCell::new(None),
            is_const: RefCell::new(false),
        }
    }

    pub fn set_name(&self, name: String) {
        self.name.replace(name);
    }

    pub fn set_ty(&self, ty: Node) {
        self.ty.replace(Some(ty));
    }

    pub fn set_expr(&self, expr: Node) {
        self.expr.replace(Some(expr));
    }

    pub fn set_is_const(&self, is_const: bool) {
        self.is_const.replace(is_const);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOpNodeKind {
    Invalid,
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    BoolAnd,
    BoolOr,
}

impl Display for BinOpNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid"),
            Self::Assign => write!(f, "="),
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Eq => write!(f, "=="),
            Self::Neq => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::Lte => write!(f, "<="),
            Self::Gte => write!(f, ">="),
            Self::And => write!(f, "&"),
            Self::Or => write!(f, "|"),
            Self::Xor => write!(f, "^"),
            Self::Shl => write!(f, "<<"),
            Self::Shr => write!(f, ">>"),
            Self::BoolAnd => write!(f, "&&"),
            Self::BoolOr => write!(f, "||"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinOpExprNode {
    pub op: RefCell<BinOpNodeKind>,
    pub lhs: RefCell<Node>,
    pub rhs: RefCell<Node>,
}

impl BinOpExprNode {
    pub fn new() -> Self {
        Self {
            op: RefCell::new(BinOpNodeKind::Invalid),
            lhs: RefCell::new(Node::new(NodeKind::Root)),
            rhs: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_op(&self, op: BinOpNodeKind) {
        self.op.replace(op);
    }

    pub fn set_lhs(&self, lhs: Node) {
        self.lhs.replace(lhs);
    }

    pub fn set_rhs(&self, rhs: Node) {
        self.rhs.replace(rhs);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnOpNodeKind {
    Invalid,
    Neg,
    Not,
    BoolNot,
}

impl Display for UnOpNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid"),
            Self::Neg => write!(f, "-"),
            Self::Not => write!(f, "~"),
            Self::BoolNot => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnOpExprNode {
    pub op: RefCell<UnOpNodeKind>,
    pub expr: RefCell<Node>,
}

impl UnOpExprNode {
    pub fn new() -> Self {
        Self {
            op: RefCell::new(UnOpNodeKind::Invalid),
            expr: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_op(&self, op: UnOpNodeKind) {
        self.op.replace(op);
    }

    pub fn set_expr(&self, expr: Node) {
        self.expr.replace(expr);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallExprNode {
    pub callee: RefCell<Node>,
    pub args: RefCell<Vec<RefCell<Node>>>,
}

impl CallExprNode {
    pub fn new() -> Self {
        Self {
            callee: RefCell::new(Node::new(NodeKind::Root)),
            args: RefCell::new(Vec::new()),
        }
    }

    pub fn set_callee(&self, callee: Node) {
        self.callee.replace(callee);
    }

    pub fn add_arg(&self, arg: Node) {
        self.args.borrow_mut().push(RefCell::new(arg));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayAccessExprNode {
    pub array: RefCell<Node>,
    pub index: RefCell<Node>,
}

impl ArrayAccessExprNode {
    pub fn new() -> Self {
        Self {
            array: RefCell::new(Node::new(NodeKind::Root)),
            index: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_array(&self, array: Node) {
        self.array.replace(array);
    }

    pub fn set_index(&self, index: Node) {
        self.index.replace(index);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExprNode {
    pub ty: RefCell<Node>,
    pub expr: RefCell<Node>,
}

impl CastExprNode {
    pub fn new() -> Self {
        Self {
            ty: RefCell::new(Node::new(NodeKind::Root)),
            expr: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_ty(&self, ty: Node) {
        self.ty.replace(ty);
    }

    pub fn set_expr(&self, expr: Node) {
        self.expr.replace(expr);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfExprNode {
    pub cond: RefCell<Node>,
    pub then: RefCell<Node>,
    pub else_: RefCell<Option<Node>>,
}

impl IfExprNode {
    pub fn new() -> Self {
        Self {
            cond: RefCell::new(Node::new(NodeKind::Root)),
            then: RefCell::new(Node::new(NodeKind::Root)),
            else_: RefCell::new(None),
        }
    }

    pub fn set_cond(&self, cond: Node) {
        self.cond.replace(cond);
    }

    pub fn set_then(&self, then: Node) {
        self.then.replace(then);
    }

    pub fn set_else(&self, else_: Node) {
        self.else_.replace(Some(else_));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmOutput {
    pub symbolic_name: RefCell<String>,
    pub constraint: RefCell<String>,
    pub var_name: RefCell<String>,
}

impl AsmOutput {
    pub fn new() -> Self {
        Self {
            symbolic_name: RefCell::new(String::new()),
            constraint: RefCell::new(String::new()),
            var_name: RefCell::new(String::new()),
        }
    }

    pub fn set_symbolic_name(&self, symbolic_name: String) {
        self.symbolic_name.replace(symbolic_name);
    }

    pub fn set_constraint(&self, constraint: String) {
        self.constraint.replace(constraint);
    }

    pub fn set_var_name(&self, var_name: String) {
        self.var_name.replace(var_name);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmInput {
    pub symbolic_name: RefCell<String>,
    pub constraint: RefCell<String>,
    pub expr: RefCell<Node>,
}

impl AsmInput {
    pub fn new() -> Self {
        Self {
            symbolic_name: RefCell::new(String::new()),
            constraint: RefCell::new(String::new()),
            expr: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_symbolic_name(&self, symbolic_name: String) {
        self.symbolic_name.replace(symbolic_name);
    }

    pub fn set_constraint(&self, constraint: String) {
        self.constraint.replace(constraint);
    }

    pub fn set_expr(&self, expr: Node) {
        self.expr.replace(expr);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrcPos {
    pub line: usize,
    pub col: usize,
}

impl SrcPos {
    pub fn new() -> Self {
        Self { line: 0, col: 0 }
    }

    pub fn set_line(&mut self, line: usize) {
        self.line = line;
    }

    pub fn set_col(&mut self, col: usize) {
        self.col = col;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmTokenKind {
    Template,
    Percent,
    Var,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmToken {
    pub kind: RefCell<AsmTokenKind>,
    pub start: RefCell<usize>,
    pub end: RefCell<usize>,
}

impl AsmToken {
    pub fn new() -> Self {
        Self {
            kind: RefCell::new(AsmTokenKind::Template),
            start: RefCell::new(0),
            end: RefCell::new(0),
        }
    }

    pub fn set_kind(&self, kind: AsmTokenKind) {
        self.kind.replace(kind);
    }

    pub fn set_start(&self, start: usize) {
        self.start.replace(start);
    }

    pub fn set_end(&self, end: usize) {
        self.end.replace(end);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmExprNode {
    pub template: RefCell<String>,
    pub outputs: RefCell<Vec<RefCell<AsmOutput>>>,
    pub inputs: RefCell<Vec<RefCell<AsmInput>>>,
    pub clobbers: RefCell<Vec<RefCell<String>>>,
    pub tokens: RefCell<Vec<RefCell<AsmToken>>>,
    pub offset_map: RefCell<Vec<SrcPos>>,
}

impl AsmExprNode {
    pub fn new() -> Self {
        Self {
            template: RefCell::new(String::new()),
            outputs: RefCell::new(Vec::new()),
            inputs: RefCell::new(Vec::new()),
            clobbers: RefCell::new(Vec::new()),
            tokens: RefCell::new(Vec::new()),
            offset_map: RefCell::new(Vec::new()),
        }
    }

    pub fn set_template(&self, template: String) {
        self.template.replace(template);
    }

    pub fn add_output(&self, output: AsmOutput) {
        self.outputs.borrow_mut().push(RefCell::new(output));
    }

    pub fn add_input(&self, input: AsmInput) {
        self.inputs.borrow_mut().push(RefCell::new(input));
    }

    pub fn add_clobber(&self, clobber: String) {
        self.clobbers.borrow_mut().push(RefCell::new(clobber));
    }

    pub fn add_token(&self, token: AsmToken) {
        self.tokens.borrow_mut().push(RefCell::new(token));
    }

    pub fn add_offset_map(&self, offset_map: SrcPos) {
        self.offset_map.borrow_mut().push(offset_map);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeData {
    None,
    Root(RootNode),
    FnProto(FnProtoNode),
    FnDef(FnDefNode),
    FnDecl(FnDeclNode),
    ParamDecl(ParamDeclNode),
    Type(TypeNode),
    Block(BlockNode),
    Extern(ExternNode),
    Import(ImportNode),
    Return(ReturnNode),
    VarDecl(VarDeclNode),
    BinOpExpr(BinOpExprNode),
    UnOpExpr(UnOpExprNode),
    CallExpr(CallExprNode),
    ArrayAccessExpr(ArrayAccessExprNode),
    CastExpr(CastExprNode),
    IfExpr(IfExprNode),
    AsmExpr(AsmExprNode),
    StrLit(String),
    NumLit(String),
    BoolLit(bool),
    Ident(String),
}

impl NodeData {
    pub fn root(&self) -> &RootNode {
        match self {
            NodeData::Root(root) => root,
            _ => panic!("NodeData::root()"),
        }
    }

    pub fn fn_proto(&self) -> &FnProtoNode {
        match self {
            NodeData::FnProto(fn_proto) => fn_proto,
            _ => panic!("NodeData::fn_proto()"),
        }
    }

    pub fn fn_def(&self) -> &FnDefNode {
        match self {
            NodeData::FnDef(fn_def) => fn_def,
            _ => panic!("NodeData::fn_def()"),
        }
    }

    pub fn fn_decl(&self) -> &FnDeclNode {
        match self {
            NodeData::FnDecl(fn_decl) => fn_decl,
            _ => panic!("NodeData::fn_decl()"),
        }
    }

    pub fn param_decl(&self) -> &ParamDeclNode {
        match self {
            NodeData::ParamDecl(param_decl) => param_decl,
            _ => panic!("NodeData::param_decl()"),
        }
    }

    pub fn type_(&self) -> &TypeNode {
        match self {
            NodeData::Type(type_) => type_,
            _ => panic!("NodeData::type_()"),
        }
    }

    pub fn block(&self) -> &BlockNode {
        match self {
            NodeData::Block(block) => block,
            _ => panic!("NodeData::block()"),
        }
    }

    pub fn extern_(&self) -> &ExternNode {
        match self {
            NodeData::Extern(extern_) => extern_,
            _ => panic!("NodeData::extern_()"),
        }
    }

    pub fn import(&self) -> &ImportNode {
        match self {
            NodeData::Import(import) => import,
            _ => panic!("NodeData::import()"),
        }
    }

    pub fn return_(&self) -> &ReturnNode {
        match self {
            NodeData::Return(return_) => return_,
            _ => panic!("NodeData::return_()"),
        }
    }

    pub fn var_decl(&self) -> &VarDeclNode {
        match self {
            NodeData::VarDecl(var_decl) => var_decl,
            _ => panic!("NodeData::var_decl()"),
        }
    }

    pub fn bin_op_expr(&self) -> &BinOpExprNode {
        match self {
            NodeData::BinOpExpr(bin_op_expr) => bin_op_expr,
            _ => panic!("NodeData::bin_op_expr()"),
        }
    }

    pub fn un_op_expr(&self) -> &UnOpExprNode {
        match self {
            NodeData::UnOpExpr(un_op_expr) => un_op_expr,
            _ => panic!("NodeData::un_op_expr()"),
        }
    }

    pub fn call_expr(&self) -> &CallExprNode {
        match self {
            NodeData::CallExpr(call_expr) => call_expr,
            _ => panic!("NodeData::call_expr()"),
        }
    }

    pub fn array_access_expr(&self) -> &ArrayAccessExprNode {
        match self {
            NodeData::ArrayAccessExpr(array_access_expr) => array_access_expr,
            _ => panic!("NodeData::array_access_expr()"),
        }
    }

    pub fn cast_expr(&self) -> &CastExprNode {
        match self {
            NodeData::CastExpr(cast_expr) => cast_expr,
            _ => panic!("NodeData::cast_expr()"),
        }
    }

    pub fn if_expr(&self) -> &IfExprNode {
        match self {
            NodeData::IfExpr(if_expr) => if_expr,
            _ => panic!("NodeData::if_expr()"),
        }
    }

    pub fn asm_expr(&self) -> &AsmExprNode {
        match self {
            NodeData::AsmExpr(asm_expr) => asm_expr,
            _ => panic!("NodeData::asm_expr()"),
        }
    }

    pub fn str_lit(&self) -> &String {
        match self {
            NodeData::StrLit(str_lit) => str_lit,
            _ => panic!("NodeData::str_lit()"),
        }
    }

    pub fn num_lit(&self) -> &String {
        match self {
            NodeData::NumLit(num_lit) => num_lit,
            _ => panic!("NodeData::num_lit()"),
        }
    }

    pub fn bool_lit(&self) -> &bool {
        match self {
            NodeData::BoolLit(bool_lit) => bool_lit,
            _ => panic!("NodeData::bool_lit()"),
        }
    }

    pub fn ident(&self) -> &String {
        match self {
            NodeData::Ident(ident) => ident,
            _ => panic!("NodeData::ident()"),
        }
    }

    pub fn new_num_lit(num_lit: String) -> NodeData {
        NodeData::NumLit(num_lit)
    }

    pub fn new_str_lit(str_lit: String) -> NodeData {
        NodeData::StrLit(str_lit)
    }

    pub fn new_bool_lit(bool_lit: bool) -> NodeData {
        NodeData::BoolLit(bool_lit)
    }

    pub fn new_ident(ident: String) -> NodeData {
        NodeData::Ident(ident)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub kind: RefCell<NodeKind>,
    pub line: RefCell<usize>,
    pub col: RefCell<usize>,
    pub data: RefCell<Box<NodeData>>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        let node = Self {
            kind: RefCell::new(kind),
            line: RefCell::new(0),
            col: RefCell::new(0),
            data: RefCell::new(Box::new(NodeData::None)),
        };
        match kind {
            NodeKind::Root => {
                node.set_data(NodeData::Root(RootNode::new()));
            }
            NodeKind::FnProto => {
                node.set_data(NodeData::FnProto(FnProtoNode::new()));
            }
            NodeKind::FnDef => {
                node.set_data(NodeData::FnDef(FnDefNode::new()));
            }
            NodeKind::FnDecl => {
                node.set_data(NodeData::FnDecl(FnDeclNode::new()));
            }
            NodeKind::ParamDecl => {
                node.set_data(NodeData::ParamDecl(ParamDeclNode::new()));
            }
            NodeKind::Type => {
                node.set_data(NodeData::Type(TypeNode::new()));
            }
            NodeKind::Block => {
                node.set_data(NodeData::Block(BlockNode::new()));
            }
            NodeKind::ExternBlock => {
                node.set_data(NodeData::Extern(ExternNode::new()));
            }
            NodeKind::Import => {
                node.set_data(NodeData::Import(ImportNode::new()));
            }
            NodeKind::Return => {
                node.set_data(NodeData::Return(ReturnNode::new()));
            }
            NodeKind::VarDecl => {
                node.set_data(NodeData::VarDecl(VarDeclNode::new()));
            }
            NodeKind::BinOpExpr => {
                node.set_data(NodeData::BinOpExpr(BinOpExprNode::new()));
            }
            NodeKind::UnaryOpExpr => {
                node.set_data(NodeData::UnOpExpr(UnOpExprNode::new()));
            }
            NodeKind::CallExpr => {
                node.set_data(NodeData::CallExpr(CallExprNode::new()));
            }
            NodeKind::ArrayAccessExpr => {
                node.set_data(NodeData::ArrayAccessExpr(ArrayAccessExprNode::new()));
            }
            NodeKind::CastExpr => {
                node.set_data(NodeData::CastExpr(CastExprNode::new()));
            }
            NodeKind::IfExpr => {
                node.set_data(NodeData::IfExpr(IfExprNode::new()));
            }
            NodeKind::AsmExpr => {
                node.set_data(NodeData::AsmExpr(AsmExprNode::new()));
            }
            NodeKind::StrLit => {
                node.set_data(NodeData::StrLit(String::new()));
            }
            NodeKind::NumLit => {
                node.set_data(NodeData::NumLit(String::new()));
            }
            NodeKind::BoolLit => {
                node.set_data(NodeData::BoolLit(false));
            }
            NodeKind::Ident => {
                node.set_data(NodeData::Ident(String::new()));
            }
            _ => {}
        }
        node
    }

    pub fn data(&self) -> Ref<Box<NodeData>> {
        self.data.borrow()
    }

    pub fn set_line(&self, line: usize) {
        self.line.replace(line);
    }

    pub fn set_col(&self, col: usize) {
        self.col.replace(col);
    }

    pub fn set_data(&self, data: NodeData) {
        self.data.replace(Box::new(data));
    }

    pub fn to_str(&self, depth: usize) -> String {
        let mut str = String::new();
        for _ in 0..depth {
            str.push_str(" ");
        }

        match self.kind.borrow().to_owned() {
            NodeKind::Root => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                for child in self.data().root().children.borrow().to_owned() {
                    str.push_str(&child.borrow().to_str(depth + 2));
                }
            }
            NodeKind::FnProto => {
                str.push_str(&format!(
                    "{} '{}'\n",
                    self.kind.borrow(),
                    self.data().fn_proto().name.borrow()
                ));
                for child in self.data().fn_proto().params.borrow().clone() {
                    str.push_str(&child.borrow().to_str(depth + 2));
                }
                str.push_str(&self.data().fn_proto().ret_type.borrow().to_str(depth + 2));
            }
            NodeKind::FnDef => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                str.push_str(&self.data().fn_def().proto.borrow().to_str(depth + 2));
                str.push_str(&self.data().fn_def().body.borrow().to_str(depth + 2));
            }
            NodeKind::FnDecl => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                str.push_str(&self.data().fn_decl().proto.borrow().to_str(depth + 2));
            }
            NodeKind::ParamDecl => {
                str.push_str(&format!(
                    "{} '{}'\n",
                    self.kind.borrow(),
                    self.data().param_decl().name.borrow()
                ));
                str.push_str(&self.data().param_decl().ty.borrow().to_str(depth + 2));
            }
            NodeKind::Type => match self.data().type_().kind.borrow().clone() {
                TypeNodeKind::Primitive => {
                    str.push_str(&format!(
                        "{} '{}'\n",
                        self.kind.borrow(),
                        self.data().type_().name.borrow()
                    ));
                }
                TypeNodeKind::Pointer => {
                    let const_or_mut = match self.data().type_().is_const.borrow().clone() {
                        true => "const",
                        false => "mut",
                    };
                    str.push_str(&format!("'{}' ptr\n", const_or_mut));
                    str.push_str(
                        &self
                            .data()
                            .type_()
                            .ty
                            .borrow()
                            .clone()
                            .unwrap()
                            .to_str(depth + 2),
                    );
                }
                TypeNodeKind::Array => {
                    str.push_str(&self.kind.borrow().to_string());
                    str.push('\n');
                    str.push_str(
                        &self
                            .data()
                            .type_()
                            .ty
                            .clone()
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .to_str(depth + 2),
                    );
                    str.push_str(
                        &self
                            .data()
                            .type_()
                            .size
                            .clone()
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .to_str(depth + 2),
                    );
                }
            },
            NodeKind::Block => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                for stmt in self.data().block().children.borrow().clone() {
                    str.push_str(&stmt.borrow().to_str(depth + 2));
                }
            }
            NodeKind::ExternBlock => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                for stmt in self.data().extern_().fn_decls.borrow().clone() {
                    str.push_str(&stmt.borrow().to_str(depth + 2));
                }
            }
            NodeKind::Import => {
                str.push_str(&format!(
                    "{} '{}'\n",
                    self.kind.borrow(),
                    self.data().import().path.borrow()
                ));
            }
            NodeKind::Return => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                str.push_str(
                    &self
                        .data()
                        .return_()
                        .expr
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .to_str(depth + 2),
                );
            }
            NodeKind::VarDecl => {
                str.push_str(&format!(
                    "{} '{}'\n",
                    self.kind.borrow(),
                    self.data().var_decl().name.borrow()
                ));
                if self.data().var_decl().ty.borrow().is_some() {
                    str.push_str(
                        &self
                            .data()
                            .var_decl()
                            .ty
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .to_str(depth + 2),
                    );
                }
                if self.data().var_decl().expr.borrow().is_some() {
                    str.push_str(
                        &self
                            .data()
                            .var_decl()
                            .expr
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .to_str(depth + 2),
                    );
                }
            }
            NodeKind::BinOpExpr => {
                str.push_str(&format!(
                    "{} {}\n",
                    self.kind.borrow(),
                    self.data().bin_op_expr().op.borrow()
                ));
                str.push_str(&self.data().bin_op_expr().lhs.borrow().to_str(depth + 2));
                str.push_str(&self.data().bin_op_expr().rhs.borrow().to_str(depth + 2));
            }
            NodeKind::UnaryOpExpr => {
                str.push_str(&format!(
                    "{} {}\n",
                    self.kind.borrow(),
                    self.data().un_op_expr().op.borrow()
                ));
                str.push_str(&self.data().un_op_expr().expr.borrow().to_str(depth + 2));
            }
            NodeKind::CallExpr => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                str.push_str(&self.data().call_expr().callee.borrow().to_str(depth + 2));
                for arg in self.data().call_expr().args.borrow().clone() {
                    str.push_str(&arg.borrow().to_str(depth + 2));
                }
            }
            NodeKind::Ident => {
                str.push_str(&format!(
                    "{} '{}'\n",
                    self.kind.borrow(),
                    self.data().ident()
                ));
            }
            NodeKind::ArrayAccessExpr => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                str.push_str(
                    &self
                        .data()
                        .array_access_expr()
                        .array
                        .borrow()
                        .to_str(depth + 2),
                );
                str.push_str(
                    &self
                        .data()
                        .array_access_expr()
                        .index
                        .borrow()
                        .to_str(depth + 2),
                );
            }
            NodeKind::CastExpr => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                str.push_str(&self.data().cast_expr().expr.borrow().to_str(depth + 2));
                str.push_str(&self.data().cast_expr().ty.borrow().to_str(depth + 2));
            }
            NodeKind::Void => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
            }
            NodeKind::IfExpr => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
                str.push_str(&self.data().if_expr().cond.borrow().to_str(depth + 2));
                str.push_str(&self.data().if_expr().then.borrow().to_str(depth + 2));
                if self.data().if_expr().else_.borrow().is_some() {
                    str.push_str(
                        &self
                            .data()
                            .if_expr()
                            .else_
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .to_str(depth + 2),
                    );
                }
            }
            NodeKind::AsmExpr => {
                str.push_str(&self.kind.borrow().to_string());
                str.push('\n');
            }
            NodeKind::StrLit => {
                str.push_str(&format!(
                    "{} '{}'\n",
                    self.kind.borrow(),
                    self.data().str_lit()
                ));
            }
            NodeKind::NumLit => {
                str.push_str(&format!(
                    "{} {}\n",
                    self.kind.borrow(),
                    self.data().num_lit()
                ));
            }
            NodeKind::BoolLit => {
                str.push_str(&format!(
                    "{} {}\n",
                    self.kind.borrow(),
                    self.data().bool_lit()
                ));
            }
        }
        str
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str(0))
    }
}
