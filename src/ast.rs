// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Display,
};

use crate::{
    codegen::{CodeGenNode, ImportTableEntry},
    dal_pointer::{DalPtr, DalWeakPtr},
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

#[derive(Clone)]
pub struct RootNode {
    pub children: Vec<DalPtr<Node>>,
}

impl RootNode {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct FnProtoNode {
    pub visib_mod: FnProtoVisibMod,
    pub name: String,
    pub params: Vec<RefCell<Node>>,
    pub ret_type: Node,
}

impl FnProtoNode {
    pub fn new() -> Self {
        Self {
            visib_mod: FnProtoVisibMod::Private,
            name: String::new(),
            params: vec![],
            ret_type: Node::new(NodeKind::Void),
        }
    }
}

#[derive(Clone)]
pub struct FnDefNode {
    pub proto: Node,
    pub body: Node,
}

impl FnDefNode {
    pub fn new() -> Self {
        Self {
            proto: Node::new(NodeKind::FnProto),
            body: Node::new(NodeKind::Block),
        }
    }
}

#[derive(Clone)]
pub struct FnDeclNode {
    pub proto: DalPtr<Node>,
}

impl FnDeclNode {
    pub fn new() -> Self {
        Self {
            proto: DalPtr::null(),
        }
    }
}

#[derive(Clone)]
pub struct ParamDeclNode {
    name: RefCell<String>,
    ty: RefCell<Node>,
}

impl ParamDeclNode {
    pub fn new() -> Self {
        Self {
            name: RefCell::new(String::new()),
            ty: RefCell::new(Node::new(NodeKind::Type)),
        }
    }

    pub fn name(&self) -> Ref<String> {
        self.name.borrow()
    }

    pub fn name_mut(&self) -> RefMut<String> {
        self.name.borrow_mut()
    }

    pub fn ty(&self) -> Ref<Node> {
        self.ty.borrow()
    }

    pub fn ty_mut(&self) -> RefMut<Node> {
        self.ty.borrow_mut()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct TypeNode {
    pub kind: TypeNodeKind,
    pub name: String,
    pub ty: DalPtr<Node>,
    pub size: DalPtr<Node>,
    pub is_const: bool,
}

impl TypeNode {
    pub fn new() -> Self {
        Self {
            kind: TypeNodeKind::Primitive,
            name: String::new(),
            ty: DalPtr::null(),
            size: DalPtr::null(),
            is_const: false,
        }
    }
}

#[derive(Clone)]
pub struct BlockNode {
    pub children: Vec<Node>,
}

impl BlockNode {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

#[derive(Clone)]
pub struct ExternNode {
    pub fn_decls: Vec<DalPtr<Node>>,
}

impl ExternNode {
    pub fn new() -> Self {
        Self { fn_decls: vec![] }
    }
}

#[derive(Clone)]
pub struct ImportNode {
    pub path: String,
}

impl ImportNode {
    pub fn new() -> Self {
        Self {
            path: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct ReturnNode {
    pub expr: Option<Node>,
}

impl ReturnNode {
    pub fn new() -> Self {
        Self { expr: None }
    }
}

#[derive(Clone)]
pub struct VarDeclNode {
    pub name: String,
    pub ty: Option<Node>,
    pub expr: Option<Node>,
    pub is_const: bool,
}

impl VarDeclNode {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            ty: None,
            expr: None,
            is_const: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct BinOpExprNode {
    pub op: BinOpNodeKind,
    pub lhs: Node,
    pub rhs: Node,
}

impl BinOpExprNode {
    pub fn new() -> Self {
        Self {
            op: BinOpNodeKind::Invalid,
            lhs: Node::new(NodeKind::Root),
            rhs: Node::new(NodeKind::Root),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct UnOpExprNode {
    pub op: UnOpNodeKind,
    pub expr: Node,
}

impl UnOpExprNode {
    pub fn new() -> Self {
        Self {
            op: UnOpNodeKind::Invalid,
            expr: Node::new(NodeKind::Root),
        }
    }
}

#[derive(Clone)]
pub struct CallExprNode {
    pub callee: Node,
    pub args: Vec<Node>,
}

impl CallExprNode {
    pub fn new() -> Self {
        Self {
            callee: Node::new(NodeKind::Root),
            args: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct ArrayAccessExprNode {
    pub array: Node,
    pub index: Node,
}

impl ArrayAccessExprNode {
    pub fn new() -> Self {
        Self {
            array: Node::new(NodeKind::Root),
            index: Node::new(NodeKind::Root),
        }
    }
}

#[derive(Clone)]
pub struct CastExprNode {
    pub ty: Node,
    pub expr: Node,
}

impl CastExprNode {
    pub fn new() -> Self {
        Self {
            ty: Node::new(NodeKind::Root),
            expr: Node::new(NodeKind::Root),
        }
    }
}

#[derive(Clone)]
pub struct IfExprNode {
    pub cond: Node,
    pub then: Node,
    pub else_: Option<Node>,
}

impl IfExprNode {
    pub fn new() -> Self {
        Self {
            cond: Node::new(NodeKind::Root),
            then: Node::new(NodeKind::Root),
            else_: None,
        }
    }
}

#[derive(Clone)]
pub struct AsmOutput {
    pub symbolic_name: String,
    pub constraint: String,
    pub var_name: String,
}

impl AsmOutput {
    pub fn new() -> Self {
        Self {
            symbolic_name: String::new(),
            constraint: String::new(),
            var_name: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct AsmInput {
    pub symbolic_name: String,
    pub constraint: String,
    pub expr: Node,
}

impl AsmInput {
    pub fn new() -> Self {
        Self {
            symbolic_name: String::new(),
            constraint: String::new(),
            expr: Node::new(NodeKind::Root),
        }
    }
}

#[derive(Clone)]
pub struct SrcPos {
    pub line: usize,
    pub col: usize,
}

impl SrcPos {
    pub fn new() -> Self {
        Self { line: 0, col: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsmTokenKind {
    Template,
    Percent,
    Var,
}

#[derive(Clone)]
pub struct AsmToken {
    pub kind: AsmTokenKind,
    pub start: usize,
    pub end: usize,
}

impl AsmToken {
    pub fn new() -> Self {
        Self {
            kind: AsmTokenKind::Template,
            start: 0,
            end: 0,
        }
    }
}

#[derive(Clone)]
pub struct AsmExprNode {
    pub template: String,
    pub outputs: Vec<AsmOutput>,
    pub inputs: Vec<AsmInput>,
    pub clobbers: Vec<String>,
    pub tokens: Vec<AsmToken>,
    pub offset_map: Vec<SrcPos>,
}

impl AsmExprNode {
    pub fn new() -> Self {
        Self {
            template: String::new(),
            outputs: Vec::new(),
            inputs: Vec::new(),
            clobbers: Vec::new(),
            tokens: Vec::new(),
            offset_map: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub enum NodeData {
    None,
    Root(RefCell<RootNode>),
    FnProto(RefCell<FnProtoNode>),
    FnDef(RefCell<FnDefNode>),
    FnDecl(RefCell<FnDeclNode>),
    ParamDecl(RefCell<ParamDeclNode>),
    Type(RefCell<TypeNode>),
    Block(RefCell<BlockNode>),
    Extern(RefCell<ExternNode>),
    Import(RefCell<ImportNode>),
    Return(RefCell<ReturnNode>),
    VarDecl(RefCell<VarDeclNode>),
    BinOpExpr(RefCell<BinOpExprNode>),
    UnOpExpr(RefCell<UnOpExprNode>),
    CallExpr(RefCell<CallExprNode>),
    ArrayAccessExpr(RefCell<ArrayAccessExprNode>),
    CastExpr(RefCell<CastExprNode>),
    IfExpr(RefCell<IfExprNode>),
    AsmExpr(RefCell<AsmExprNode>),
    StrLit(RefCell<String>),
    NumLit(RefCell<String>),
    BoolLit(RefCell<bool>),
    Ident(RefCell<String>),
}

impl NodeData {
    pub fn root(&self) -> Ref<RootNode> {
        match self {
            NodeData::Root(root) => root.borrow(),
            _ => panic!("NodeData::root()"),
        }
    }

    pub fn root_mut(&self) -> RefMut<RootNode> {
        match self {
            NodeData::Root(root) => root.borrow_mut(),
            _ => panic!("NodeData::root_mut()"),
        }
    }

    pub fn fn_proto(&self) -> Ref<FnProtoNode> {
        match self {
            NodeData::FnProto(fn_proto) => fn_proto.borrow(),
            _ => panic!("NodeData::fn_proto()"),
        }
    }

    pub fn fn_proto_mut(&self) -> RefMut<FnProtoNode> {
        match self {
            NodeData::FnProto(fn_proto) => fn_proto.borrow_mut(),
            _ => panic!("NodeData::fn_proto_mut()"),
        }
    }

    pub fn fn_def(&self) -> Ref<FnDefNode> {
        match self {
            NodeData::FnDef(fn_def) => fn_def.borrow(),
            _ => panic!("NodeData::fn_def()"),
        }
    }

    pub fn fn_def_mut(&self) -> RefMut<FnDefNode> {
        match self {
            NodeData::FnDef(fn_def) => fn_def.borrow_mut(),
            _ => panic!("NodeData::fn_def_mut()"),
        }
    }

    pub fn fn_decl(&self) -> Ref<FnDeclNode> {
        match self {
            NodeData::FnDecl(fn_decl) => fn_decl.borrow(),
            _ => panic!("NodeData::fn_decl()"),
        }
    }

    pub fn fn_decl_mut(&self) -> RefMut<FnDeclNode> {
        match self {
            NodeData::FnDecl(fn_decl) => fn_decl.borrow_mut(),
            _ => panic!("NodeData::fn_decl_mut()"),
        }
    }

    pub fn param_decl(&self) -> Ref<ParamDeclNode> {
        match self {
            NodeData::ParamDecl(param_decl) => param_decl.borrow(),
            _ => panic!("NodeData::param_decl()"),
        }
    }

    pub fn param_decl_mut(&self) -> RefMut<ParamDeclNode> {
        match self {
            NodeData::ParamDecl(param_decl) => param_decl.borrow_mut(),
            _ => panic!("NodeData::param_decl_mut()"),
        }
    }

    pub fn type_(&self) -> Ref<TypeNode> {
        match self {
            NodeData::Type(type_) => type_.borrow(),
            _ => panic!("NodeData::type_()"),
        }
    }

    pub fn type_mut(&self) -> RefMut<TypeNode> {
        match self {
            NodeData::Type(type_) => type_.borrow_mut(),
            _ => panic!("NodeData::type_mut()"),
        }
    }

    pub fn block(&self) -> Ref<BlockNode> {
        match self {
            NodeData::Block(block) => block.borrow(),
            _ => panic!("NodeData::block()"),
        }
    }

    pub fn block_mut(&self) -> RefMut<BlockNode> {
        match self {
            NodeData::Block(block) => block.borrow_mut(),
            _ => panic!("NodeData::block_mut()"),
        }
    }

    pub fn extern_(&self) -> Ref<ExternNode> {
        match self {
            NodeData::Extern(extern_) => extern_.borrow(),
            _ => panic!("NodeData::extern_()"),
        }
    }

    pub fn extern_mut(&self) -> RefMut<ExternNode> {
        match self {
            NodeData::Extern(extern_) => extern_.borrow_mut(),
            _ => panic!("NodeData::extern_mut()"),
        }
    }

    pub fn import(&self) -> Ref<ImportNode> {
        match self {
            NodeData::Import(import) => import.borrow(),
            _ => panic!("NodeData::import()"),
        }
    }

    pub fn import_mut(&self) -> RefMut<ImportNode> {
        match self {
            NodeData::Import(import) => import.borrow_mut(),
            _ => panic!("NodeData::import_mut()"),
        }
    }

    pub fn return_(&self) -> Ref<ReturnNode> {
        match self {
            NodeData::Return(return_) => return_.borrow(),
            _ => panic!("NodeData::return_()"),
        }
    }

    pub fn return_mut(&self) -> RefMut<ReturnNode> {
        match self {
            NodeData::Return(return_) => return_.borrow_mut(),
            _ => panic!("NodeData::return_mut()"),
        }
    }

    pub fn var_decl(&self) -> Ref<VarDeclNode> {
        match self {
            NodeData::VarDecl(var_decl) => var_decl.borrow(),
            _ => panic!("NodeData::var_decl()"),
        }
    }

    pub fn var_decl_mut(&self) -> RefMut<VarDeclNode> {
        match self {
            NodeData::VarDecl(var_decl) => var_decl.borrow_mut(),
            _ => panic!("NodeData::var_decl_mut()"),
        }
    }

    pub fn bin_op_expr(&self) -> Ref<BinOpExprNode> {
        match self {
            NodeData::BinOpExpr(bin_op_expr) => bin_op_expr.borrow(),
            _ => panic!("NodeData::bin_op_expr()"),
        }
    }

    pub fn bin_op_expr_mut(&self) -> RefMut<BinOpExprNode> {
        match self {
            NodeData::BinOpExpr(bin_op_expr) => bin_op_expr.borrow_mut(),
            _ => panic!("NodeData::bin_op_expr_mut()"),
        }
    }

    pub fn un_op_expr(&self) -> Ref<UnOpExprNode> {
        match self {
            NodeData::UnOpExpr(un_op_expr) => un_op_expr.borrow(),
            _ => panic!("NodeData::un_op_expr()"),
        }
    }

    pub fn un_op_expr_mut(&self) -> RefMut<UnOpExprNode> {
        match self {
            NodeData::UnOpExpr(un_op_expr) => un_op_expr.borrow_mut(),
            _ => panic!("NodeData::un_op_expr_mut()"),
        }
    }

    pub fn call_expr(&self) -> Ref<CallExprNode> {
        match self {
            NodeData::CallExpr(call_expr) => call_expr.borrow(),
            _ => panic!("NodeData::call_expr()"),
        }
    }

    pub fn call_expr_mut(&self) -> RefMut<CallExprNode> {
        match self {
            NodeData::CallExpr(call_expr) => call_expr.borrow_mut(),
            _ => panic!("NodeData::call_expr_mut()"),
        }
    }

    pub fn array_access_expr(&self) -> Ref<ArrayAccessExprNode> {
        match self {
            NodeData::ArrayAccessExpr(array_access_expr) => array_access_expr.borrow(),
            _ => panic!("NodeData::array_access_expr()"),
        }
    }

    pub fn array_access_expr_mut(&self) -> RefMut<ArrayAccessExprNode> {
        match self {
            NodeData::ArrayAccessExpr(array_access_expr) => array_access_expr.borrow_mut(),
            _ => panic!("NodeData::array_access_expr_mut()"),
        }
    }

    pub fn cast_expr(&self) -> Ref<CastExprNode> {
        match self {
            NodeData::CastExpr(cast_expr) => cast_expr.borrow(),
            _ => panic!("NodeData::cast_expr()"),
        }
    }

    pub fn cast_expr_mut(&self) -> RefMut<CastExprNode> {
        match self {
            NodeData::CastExpr(cast_expr) => cast_expr.borrow_mut(),
            _ => panic!("NodeData::cast_expr_mut()"),
        }
    }

    pub fn if_expr(&self) -> Ref<IfExprNode> {
        match self {
            NodeData::IfExpr(if_expr) => if_expr.borrow(),
            _ => panic!("NodeData::if_expr()"),
        }
    }

    pub fn if_expr_mut(&self) -> RefMut<IfExprNode> {
        match self {
            NodeData::IfExpr(if_expr) => if_expr.borrow_mut(),
            _ => panic!("NodeData::if_expr_mut()"),
        }
    }

    pub fn asm_expr(&self) -> Ref<AsmExprNode> {
        match self {
            NodeData::AsmExpr(asm_expr) => asm_expr.borrow(),
            _ => panic!("NodeData::asm_expr()"),
        }
    }

    pub fn asm_expr_mut(&self) -> RefMut<AsmExprNode> {
        match self {
            NodeData::AsmExpr(asm_expr) => asm_expr.borrow_mut(),
            _ => panic!("NodeData::asm_expr_mut()"),
        }
    }

    pub fn str_lit(&self) -> Ref<String> {
        match self {
            NodeData::StrLit(str_lit) => str_lit.borrow(),
            _ => panic!("NodeData::str_lit()"),
        }
    }

    pub fn str_lit_mut(&self) -> RefMut<String> {
        match self {
            NodeData::StrLit(str_lit) => str_lit.borrow_mut(),
            _ => panic!("NodeData::str_lit_mut()"),
        }
    }

    pub fn num_lit(&self) -> Ref<String> {
        match self {
            NodeData::NumLit(num_lit) => num_lit.borrow(),
            _ => panic!("NodeData::num_lit()"),
        }
    }

    pub fn num_lit_mut(&self) -> RefMut<String> {
        match self {
            NodeData::NumLit(num_lit) => num_lit.borrow_mut(),
            _ => panic!("NodeData::num_lit_mut()"),
        }
    }

    pub fn bool_lit(&self) -> Ref<bool> {
        match self {
            NodeData::BoolLit(bool_lit) => bool_lit.borrow(),
            _ => panic!("NodeData::bool_lit()"),
        }
    }

    pub fn bool_lit_mut(&self) -> RefMut<bool> {
        match self {
            NodeData::BoolLit(bool_lit) => bool_lit.borrow_mut(),
            _ => panic!("NodeData::bool_lit_mut()"),
        }
    }

    pub fn ident(&self) -> Ref<String> {
        match self {
            NodeData::Ident(ident) => ident.borrow(),
            _ => panic!("NodeData::ident()"),
        }
    }

    pub fn ident_mut(&self) -> RefMut<String> {
        match self {
            NodeData::Ident(ident) => ident.borrow_mut(),
            _ => panic!("NodeData::ident_mut()"),
        }
    }

    pub fn new_num_lit(num_lit: String) -> NodeData {
        NodeData::NumLit(RefCell::new(num_lit))
    }

    pub fn new_str_lit(str_lit: String) -> NodeData {
        NodeData::StrLit(RefCell::new(str_lit))
    }

    pub fn new_bool_lit(bool_lit: bool) -> NodeData {
        NodeData::BoolLit(RefCell::new(bool_lit))
    }

    pub fn new_ident(ident: String) -> NodeData {
        NodeData::Ident(RefCell::new(ident))
    }
}

#[derive(Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub line: usize,
    pub col: usize,
    pub data: Box<NodeData>,
    pub owner: DalWeakPtr<ImportTableEntry>,
    pub codegen_node: DalPtr<CodeGenNode>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        let mut node = Self {
            kind,
            line: 0,
            col: 0,
            data: Box::new(NodeData::None),
            owner: DalWeakPtr::new(),
            codegen_node: DalPtr::null(),
        };
        match kind {
            NodeKind::Root => {
                node.data = Box::new(NodeData::Root(RefCell::new(RootNode::new())));
            }
            NodeKind::FnProto => {
                node.data = Box::new(NodeData::FnProto(RefCell::new(FnProtoNode::new())));
            }
            NodeKind::FnDef => {
                node.data = Box::new(NodeData::FnDef(RefCell::new(FnDefNode::new())));
            }
            NodeKind::FnDecl => {
                node.data = Box::new(NodeData::FnDecl(RefCell::new(FnDeclNode::new())));
            }
            NodeKind::ParamDecl => {
                node.data = Box::new(NodeData::ParamDecl(RefCell::new(ParamDeclNode::new())));
            }
            NodeKind::Type => {
                node.data = Box::new(NodeData::Type(RefCell::new(TypeNode::new())));
            }
            NodeKind::Block => {
                node.data = Box::new(NodeData::Block(RefCell::new(BlockNode::new())));
            }
            NodeKind::ExternBlock => {
                node.data = Box::new(NodeData::Extern(RefCell::new(ExternNode::new())));
            }
            NodeKind::Import => {
                node.data = Box::new(NodeData::Import(RefCell::new(ImportNode::new())));
            }
            NodeKind::Return => {
                node.data = Box::new(NodeData::Return(RefCell::new(ReturnNode::new())));
            }
            NodeKind::VarDecl => {
                node.data = Box::new(NodeData::VarDecl(RefCell::new(VarDeclNode::new())));
            }
            NodeKind::BinOpExpr => {
                node.data = Box::new(NodeData::BinOpExpr(RefCell::new(BinOpExprNode::new())));
            }
            NodeKind::UnaryOpExpr => {
                node.data = Box::new(NodeData::UnOpExpr(RefCell::new(UnOpExprNode::new())));
            }
            NodeKind::CallExpr => {
                node.data = Box::new(NodeData::CallExpr(RefCell::new(CallExprNode::new())));
            }
            NodeKind::ArrayAccessExpr => {
                node.data = Box::new(NodeData::ArrayAccessExpr(RefCell::new(
                    ArrayAccessExprNode::new(),
                )));
            }
            NodeKind::CastExpr => {
                node.data = Box::new(NodeData::CastExpr(RefCell::new(CastExprNode::new())));
            }
            NodeKind::IfExpr => {
                node.data = Box::new(NodeData::IfExpr(RefCell::new(IfExprNode::new())));
            }
            NodeKind::AsmExpr => {
                node.data = Box::new(NodeData::AsmExpr(RefCell::new(AsmExprNode::new())));
            }
            NodeKind::StrLit => {
                node.data = Box::new(NodeData::StrLit(RefCell::new(String::new())));
            }
            NodeKind::NumLit => {
                node.data = Box::new(NodeData::NumLit(RefCell::new(String::new())));
            }
            NodeKind::BoolLit => {
                node.data = Box::new(NodeData::BoolLit(RefCell::new(false)));
            }
            NodeKind::Ident => {
                node.data = Box::new(NodeData::Ident(RefCell::new(String::new())));
            }
            _ => {}
        }
        node
    }

    pub fn owner(&self) -> DalPtr<ImportTableEntry> {
        self.owner.upgrade().unwrap()
    }

    pub fn to_str(&self, depth: usize) -> String {
        let mut str = String::new();
        for _ in 0..depth {
            str.push_str(" ");
        }

        match self.kind {
            NodeKind::Root => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                for child in &self.data.root().children {
                    str.push_str(&child.borrow().to_str(depth + 2));
                }
            }
            NodeKind::FnProto => {
                str.push_str(&format!("{} '{}'\n", self.kind, self.data.fn_proto().name));
                for child in &self.data.fn_proto().params {
                    str.push_str(&child.borrow().to_str(depth + 2));
                }
                str.push_str(&self.data.fn_proto().ret_type.to_str(depth + 2));
            }
            NodeKind::FnDef => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                str.push_str(&self.data.fn_def().proto.to_str(depth + 2));
                str.push_str(&self.data.fn_def().body.to_str(depth + 2));
            }
            NodeKind::FnDecl => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                str.push_str(&self.data.fn_decl().proto.borrow().to_str(depth + 2));
            }
            NodeKind::ParamDecl => {
                str.push_str(&format!(
                    "{} '{}'\n",
                    self.kind,
                    self.data.param_decl().name.borrow()
                ));
                str.push_str(&self.data.param_decl().ty.borrow().to_str(depth + 2));
            }
            NodeKind::Type => match self.data.type_().kind {
                TypeNodeKind::Primitive => {
                    str.push_str(&format!("{} '{}'\n", self.kind, self.data.type_().name));
                }
                TypeNodeKind::Pointer => {
                    let const_or_mut = match self.data.type_().is_const {
                        true => "const",
                        false => "mut",
                    };
                    str.push_str(&format!("'{}' ptr\n", const_or_mut));
                    str.push_str(&self.data.type_().ty.borrow().to_str(depth + 2));
                }
                TypeNodeKind::Array => {
                    str.push_str(&self.kind.to_string());
                    str.push('\n');
                    str.push_str(&self.data.type_().ty.borrow().to_str(depth + 2));
                    str.push_str(&self.data.type_().size.borrow().to_str(depth + 2));
                }
            },
            NodeKind::Block => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                for stmt in &self.data.block().children {
                    str.push_str(&stmt.to_str(depth + 2));
                }
            }
            NodeKind::ExternBlock => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                for stmt in &self.data.extern_().fn_decls {
                    str.push_str(&stmt.borrow().to_str(depth + 2));
                }
            }
            NodeKind::Import => {
                str.push_str(&format!("{} '{}'\n", self.kind, self.data.import().path));
            }
            NodeKind::Return => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                str.push_str(&self.data.return_().expr.as_ref().unwrap().to_str(depth + 2));
            }
            NodeKind::VarDecl => {
                str.push_str(&format!("{} '{}'\n", self.kind, self.data.var_decl().name));
                if self.data.var_decl().ty.is_some() {
                    str.push_str(&self.data.var_decl().ty.as_ref().unwrap().to_str(depth + 2));
                }
                if self.data.var_decl().expr.is_some() {
                    str.push_str(
                        &self
                            .data
                            .var_decl()
                            .expr
                            .as_ref()
                            .unwrap()
                            .to_str(depth + 2),
                    );
                }
            }
            NodeKind::BinOpExpr => {
                str.push_str(&format!("{} {}\n", self.kind, self.data.bin_op_expr().op));
                str.push_str(&self.data.bin_op_expr().lhs.to_str(depth + 2));
                str.push_str(&self.data.bin_op_expr().rhs.to_str(depth + 2));
            }
            NodeKind::UnaryOpExpr => {
                str.push_str(&format!("{} {}\n", self.kind, self.data.un_op_expr().op));
                str.push_str(&self.data.un_op_expr().expr.to_str(depth + 2));
            }
            NodeKind::CallExpr => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                str.push_str(&self.data.call_expr().callee.to_str(depth + 2));
                for arg in self.data.call_expr().args.clone() {
                    str.push_str(&arg.to_str(depth + 2));
                }
            }
            NodeKind::Ident => {
                str.push_str(&format!("{} '{}'\n", self.kind, self.data.ident()));
            }
            NodeKind::ArrayAccessExpr => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                str.push_str(&self.data.array_access_expr().array.to_str(depth + 2));
                str.push_str(&self.data.array_access_expr().index.to_str(depth + 2));
            }
            NodeKind::CastExpr => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                str.push_str(&self.data.cast_expr().expr.to_str(depth + 2));
                str.push_str(&self.data.cast_expr().ty.to_str(depth + 2));
            }
            NodeKind::Void => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
            }
            NodeKind::IfExpr => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
                str.push_str(&self.data.if_expr().cond.to_str(depth + 2));
                str.push_str(&self.data.if_expr().then.to_str(depth + 2));
                if self.data.if_expr().else_.is_some() {
                    str.push_str(
                        &self
                            .data
                            .if_expr()
                            .else_
                            .as_ref()
                            .unwrap()
                            .to_str(depth + 2),
                    );
                }
            }
            NodeKind::AsmExpr => {
                str.push_str(&self.kind.to_string());
                str.push('\n');
            }
            NodeKind::StrLit => {
                str.push_str(&format!("{} '{}'\n", self.kind, self.data.str_lit()));
            }
            NodeKind::NumLit => {
                str.push_str(&format!("{} {}\n", self.kind, self.data.num_lit()));
            }
            NodeKind::BoolLit => {
                str.push_str(&format!("{} {}\n", self.kind, self.data.bool_lit()));
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
