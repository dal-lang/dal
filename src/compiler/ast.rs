// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use std::cell::RefCell;

use crate::codegen::analyze::ImportTableEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Root,
    FnProto,
    FnDef,
    FnDecl,
    ParamDecl,
    Type,
    Block,
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

impl NodeKind {
    pub fn to_str(&self) -> &str {
        match self {
            NodeKind::Root => "Root",
            NodeKind::FnProto => "FnProto",
            NodeKind::FnDef => "FnDef",
            NodeKind::FnDecl => "FnDecl",
            NodeKind::ParamDecl => "ParamDecl",
            NodeKind::Type => "Type",
            NodeKind::Block => "Block",
            NodeKind::Import => "Import",
            NodeKind::Return => "Return",
            NodeKind::VarDecl => "VarDecl",
            NodeKind::BinOpExpr => "BinOpExpr",
            NodeKind::UnaryOpExpr => "UnaryOpExpr",
            NodeKind::CallExpr => "CallExpr",
            NodeKind::Ident => "Ident",
            NodeKind::ArrayAccessExpr => "ArrayAccessExpr",
            NodeKind::CastExpr => "CastExpr",
            NodeKind::Void => "Void",
            NodeKind::IfExpr => "IfExpr",
            NodeKind::AsmExpr => "AsmExpr",
            NodeKind::StrLit => "StrLit",
            NodeKind::NumLit => "NumLit",
            NodeKind::BoolLit => "BoolLit",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeRoot {
    pub children: RefCell<Vec<RefCell<Node>>>,
}

impl NodeRoot {
    pub fn new() -> Self {
        Self {
            children: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, node: Node) {
        self.children.borrow_mut().push(RefCell::new(node));
    }

    pub fn set_children(&self, children: Vec<RefCell<Node>>) {
        *self.children.borrow_mut() = children;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FnProtoVisibMod {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFnProto {
    pub visib_mod: RefCell<FnProtoVisibMod>,
    pub name: RefCell<String>,
    pub params: RefCell<Vec<RefCell<Node>>>,
    pub ret_type: RefCell<Node>,
}

impl NodeFnProto {
    pub fn new() -> Self {
        Self {
            visib_mod: RefCell::new(FnProtoVisibMod::Public),
            name: RefCell::new(String::new()),
            params: RefCell::new(Vec::new()),
            ret_type: RefCell::new(Node::new(NodeKind::Void)),
        }
    }

    pub fn set_visib_mod(&self, visib_mod: FnProtoVisibMod) {
        *self.visib_mod.borrow_mut() = visib_mod;
    }

    pub fn set_name(&self, name: String) {
        *self.name.borrow_mut() = name;
    }

    pub fn push_param(&self, param: Node) {
        self.params.borrow_mut().push(RefCell::new(param));
    }

    pub fn set_ret_type(&self, ret_type: Node) {
        *self.ret_type.borrow_mut() = ret_type;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFnDef {
    pub proto: RefCell<Node>,
    pub body: RefCell<Node>,
}

impl NodeFnDef {
    pub fn new() -> Self {
        Self {
            proto: RefCell::new(Node::new(NodeKind::Root)),
            body: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_proto(&self, proto: Node) {
        *self.proto.borrow_mut() = proto;
    }

    pub fn set_body(&self, body: Node) {
        *self.body.borrow_mut() = body;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFnDecl {
    pub proto: RefCell<Node>,
}

impl NodeFnDecl {
    pub fn new() -> Self {
        Self {
            proto: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_proto(&self, proto: Node) {
        *self.proto.borrow_mut() = proto;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeParamDecl {
    pub name: RefCell<String>,
    pub param_type: RefCell<Node>,
}

impl NodeParamDecl {
    pub fn new() -> Self {
        Self {
            name: RefCell::new(String::new()),
            param_type: RefCell::new(Node::new(NodeKind::Void)),
        }
    }

    pub fn set_name(&self, name: String) {
        *self.name.borrow_mut() = name;
    }

    pub fn set_param_type(&self, param_type: Node) {
        *self.param_type.borrow_mut() = param_type;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Primitive,
    Pointer,
    Array,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeType {
    pub kind: RefCell<TypeKind>,
    pub name: RefCell<String>,
    pub child_type: RefCell<Option<Node>>,
    pub array_size: RefCell<Option<Node>>,
    pub is_const: RefCell<bool>,
}

impl NodeType {
    pub fn new() -> Self {
        Self {
            kind: RefCell::new(TypeKind::Primitive),
            name: RefCell::new(String::new()),
            child_type: RefCell::new(None),
            array_size: RefCell::new(None),
            is_const: RefCell::new(false),
        }
    }

    pub fn set_kind(&self, kind: TypeKind) {
        *self.kind.borrow_mut() = kind;
    }

    pub fn set_name(&self, name: String) {
        *self.name.borrow_mut() = name;
    }

    pub fn set_child_type(&self, child_type: Node) {
        *self.child_type.borrow_mut() = Some(child_type);
    }

    pub fn set_array_size(&self, array_size: Node) {
        *self.array_size.borrow_mut() = Some(array_size);
    }

    pub fn set_is_const(&self, is_const: bool) {
        *self.is_const.borrow_mut() = is_const;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeBlock {
    pub children: RefCell<Vec<RefCell<Node>>>,
}

impl NodeBlock {
    pub fn new() -> Self {
        Self {
            children: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, node: Node) {
        self.children.borrow_mut().push(RefCell::new(node));
    }

    pub fn set_children(&self, children: Vec<RefCell<Node>>) {
        *self.children.borrow_mut() = children;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeImport {
    pub path: RefCell<String>,
}

impl NodeImport {
    pub fn new() -> Self {
        Self {
            path: RefCell::new(String::new()),
        }
    }

    pub fn set_path(&self, path: String) {
        *self.path.borrow_mut() = path;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeReturn {
    pub expr: RefCell<Option<Node>>,
}

impl NodeReturn {
    pub fn new() -> Self {
        Self {
            expr: RefCell::new(None),
        }
    }

    pub fn set_expr(&self, expr: Node) {
        *self.expr.borrow_mut() = Some(expr);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeVarDecl {
    pub name: RefCell<String>,
    pub is_const: RefCell<bool>,
    pub var_type: RefCell<Option<Node>>,
    pub expr: RefCell<Option<Node>>,
}

impl NodeVarDecl {
    pub fn new() -> Self {
        Self {
            name: RefCell::new(String::new()),
            is_const: RefCell::new(false),
            var_type: RefCell::new(None),
            expr: RefCell::new(None),
        }
    }

    pub fn set_name(&self, name: String) {
        *self.name.borrow_mut() = name;
    }

    pub fn set_is_const(&self, is_const: bool) {
        *self.is_const.borrow_mut() = is_const;
    }

    pub fn set_var_type(&self, var_type: Node) {
        *self.var_type.borrow_mut() = Some(var_type);
    }

    pub fn set_expr(&self, expr: Node) {
        *self.expr.borrow_mut() = Some(expr);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOpKind {
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

impl BinOpKind {
    pub fn to_str(&self) -> &str {
        match self {
            BinOpKind::Invalid => "Invalid",
            BinOpKind::Assign => "=",
            BinOpKind::Add => "+",
            BinOpKind::Sub => "-",
            BinOpKind::Mul => "*",
            BinOpKind::Div => "/",
            BinOpKind::Mod => "%",
            BinOpKind::Eq => "==",
            BinOpKind::Neq => "!=",
            BinOpKind::Lt => "<",
            BinOpKind::Gt => ">",
            BinOpKind::Lte => "<=",
            BinOpKind::Gte => ">=",
            BinOpKind::And => "&",
            BinOpKind::Or => "|",
            BinOpKind::Xor => "^",
            BinOpKind::Shl => "<<",
            BinOpKind::Shr => ">>",
            BinOpKind::BoolAnd => "&&",
            BinOpKind::BoolOr => "||",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeBinOpExpr {
    pub op: RefCell<BinOpKind>,
    pub lhs: RefCell<Node>,
    pub rhs: RefCell<Node>,
}

impl NodeBinOpExpr {
    pub fn new() -> Self {
        Self {
            op: RefCell::new(BinOpKind::Invalid),
            lhs: RefCell::new(Node::new(NodeKind::Root)),
            rhs: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_op(&self, op: BinOpKind) {
        *self.op.borrow_mut() = op;
    }

    pub fn set_lhs(&self, lhs: Node) {
        *self.lhs.borrow_mut() = lhs;
    }

    pub fn set_rhs(&self, rhs: Node) {
        *self.rhs.borrow_mut() = rhs;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpKind {
    Invalid,
    Neg,
    Not,
    BoolNot,
}

impl UnaryOpKind {
    pub fn to_str(&self) -> &str {
        match self {
            UnaryOpKind::Invalid => "Invalid",
            UnaryOpKind::Neg => "-",
            UnaryOpKind::Not => "~",
            UnaryOpKind::BoolNot => "!",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeUnaryOpExpr {
    pub op: RefCell<UnaryOpKind>,
    pub expr: RefCell<Node>,
}

impl NodeUnaryOpExpr {
    pub fn new() -> Self {
        Self {
            op: RefCell::new(UnaryOpKind::Invalid),
            expr: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_op(&self, op: UnaryOpKind) {
        *self.op.borrow_mut() = op;
    }

    pub fn set_expr(&self, expr: Node) {
        *self.expr.borrow_mut() = expr;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeCallExpr {
    pub callee: RefCell<Node>,
    pub args: RefCell<Vec<RefCell<Node>>>,
}

impl NodeCallExpr {
    pub fn new() -> Self {
        Self {
            callee: RefCell::new(Node::new(NodeKind::Root)),
            args: RefCell::new(Vec::new()),
        }
    }

    pub fn set_callee(&self, callee: Node) {
        *self.callee.borrow_mut() = callee;
    }

    pub fn add_arg(&self, arg: Node) {
        self.args.borrow_mut().push(RefCell::new(arg));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeArrayAccessExpr {
    pub array: RefCell<Node>,
    pub index: RefCell<Node>,
}

impl NodeArrayAccessExpr {
    pub fn new() -> Self {
        Self {
            array: RefCell::new(Node::new(NodeKind::Root)),
            index: RefCell::new(Node::new(NodeKind::Root)),
        }
    }

    pub fn set_array(&self, array: Node) {
        *self.array.borrow_mut() = array;
    }

    pub fn set_index(&self, index: Node) {
        *self.index.borrow_mut() = index;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExpr {
    pub expr: RefCell<Node>,
    pub cast_type: RefCell<Option<Node>>,
}

impl CastExpr {
    pub fn new() -> Self {
        Self {
            expr: RefCell::new(Node::new(NodeKind::Root)),
            cast_type: RefCell::new(None),
        }
    }

    pub fn set_expr(&self, expr: Node) {
        *self.expr.borrow_mut() = expr;
    }

    pub fn set_cast_type(&self, cast_type: Node) {
        *self.cast_type.borrow_mut() = Some(cast_type);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeIfExpr {
    pub cond: RefCell<Node>,
    pub then: RefCell<Node>,
    pub else_: RefCell<Option<Node>>,
}

impl NodeIfExpr {
    pub fn new() -> NodeIfExpr {
        NodeIfExpr {
            cond: RefCell::new(Node::new(NodeKind::Root)),
            then: RefCell::new(Node::new(NodeKind::Root)),
            else_: RefCell::new(None),
        }
    }

    pub fn set_cond(&self, cond: Node) {
        *self.cond.borrow_mut() = cond;
    }

    pub fn set_then(&self, then: Node) {
        *self.then.borrow_mut() = then;
    }

    pub fn set_else(&self, else_: Node) {
        *self.else_.borrow_mut() = Some(else_);
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
        *self.symbolic_name.borrow_mut() = symbolic_name;
    }

    pub fn set_constraint(&self, constraint: String) {
        *self.constraint.borrow_mut() = constraint;
    }

    pub fn set_var_name(&self, var_name: String) {
        *self.var_name.borrow_mut() = var_name;
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
        *self.symbolic_name.borrow_mut() = symbolic_name;
    }

    pub fn set_constraint(&self, constraint: String) {
        *self.constraint.borrow_mut() = constraint;
    }

    pub fn set_expr(&self, expr: Node) {
        *self.expr.borrow_mut() = expr;
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
        *self.kind.borrow_mut() = kind;
    }

    pub fn set_start(&self, start: usize) {
        *self.start.borrow_mut() = start;
    }

    pub fn set_end(&self, end: usize) {
        *self.end.borrow_mut() = end;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmExpr {
    pub asm_template: RefCell<String>,
    pub offset_map: RefCell<Vec<SrcPos>>,
    pub token_list: RefCell<Vec<AsmToken>>,
    pub output_list: RefCell<Vec<AsmOutput>>,
    pub input_list: RefCell<Vec<AsmInput>>,
    pub clobber_list: RefCell<Vec<String>>,
}

impl AsmExpr {
    pub fn new() -> Self {
        Self {
            asm_template: RefCell::new(String::new()),
            offset_map: RefCell::new(Vec::new()),
            token_list: RefCell::new(Vec::new()),
            output_list: RefCell::new(Vec::new()),
            input_list: RefCell::new(Vec::new()),
            clobber_list: RefCell::new(Vec::new()),
        }
    }

    pub fn set_asm_template(&self, asm_template: String) {
        *self.asm_template.borrow_mut() = asm_template;
    }

    pub fn set_offset_map(&self, offset_map: Vec<SrcPos>) {
        *self.offset_map.borrow_mut() = offset_map;
    }

    pub fn set_token_list(&self, token_list: Vec<AsmToken>) {
        *self.token_list.borrow_mut() = token_list;
    }

    pub fn set_output_list(&self, output_list: Vec<AsmOutput>) {
        *self.output_list.borrow_mut() = output_list;
    }

    pub fn set_input_list(&self, input_list: Vec<AsmInput>) {
        *self.input_list.borrow_mut() = input_list;
    }

    pub fn set_clobber_list(&self, clobber_list: Vec<String>) {
        *self.clobber_list.borrow_mut() = clobber_list;
    }

    pub fn add_offset_map(&self, offset_map: SrcPos) {
        self.offset_map.borrow_mut().push(offset_map);
    }

    pub fn add_token_list(&self, token_list: AsmToken) {
        self.token_list.borrow_mut().push(token_list);
    }

    pub fn add_output_list(&self, output_list: AsmOutput) {
        self.output_list.borrow_mut().push(output_list);
    }

    pub fn add_input_list(&self, input_list: AsmInput) {
        self.input_list.borrow_mut().push(input_list);
    }

    pub fn add_clobber_list(&self, clobber_list: String) {
        self.clobber_list.borrow_mut().push(clobber_list);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeData {
    None,
    Root(RefCell<NodeRoot>),
    FnProto(RefCell<NodeFnProto>),
    FnDef(RefCell<NodeFnDef>),
    FnDecl(RefCell<NodeFnDecl>),
    ParamDecl(RefCell<NodeParamDecl>),
    Type(RefCell<NodeType>),
    Block(RefCell<NodeBlock>),
    Import(RefCell<NodeImport>),
    Return(RefCell<NodeReturn>),
    VarDecl(RefCell<NodeVarDecl>),
    BinOpExpr(RefCell<NodeBinOpExpr>),
    UnaryOpExpr(RefCell<NodeUnaryOpExpr>),
    CallExpr(RefCell<NodeCallExpr>),
    ArrayAccessExpr(RefCell<NodeArrayAccessExpr>),
    CastExpr(RefCell<CastExpr>),
    IfExpr(RefCell<NodeIfExpr>),
    AsmExpr(RefCell<AsmExpr>),
    StrLit(RefCell<String>),
    NumLit(RefCell<String>),
    BoolLit(RefCell<bool>),
    Ident(RefCell<String>),
}

impl NodeData {
    pub fn root(&self) -> &RefCell<NodeRoot> {
        match self {
            NodeData::Root(root) => root,
            _ => panic!("NodeData::root() called on non-root node"),
        }
    }

    pub fn fn_proto(&self) -> &RefCell<NodeFnProto> {
        match self {
            NodeData::FnProto(fn_proto) => fn_proto,
            _ => panic!("NodeData::fn_proto() called on non-fn_proto node"),
        }
    }

    pub fn fn_def(&self) -> &RefCell<NodeFnDef> {
        match self {
            NodeData::FnDef(fn_def) => fn_def,
            _ => panic!("NodeData::fn_def() called on non-fn_def node"),
        }
    }

    pub fn fn_decl(&self) -> &RefCell<NodeFnDecl> {
        match self {
            NodeData::FnDecl(fn_decl) => fn_decl,
            _ => panic!("NodeData::fn_decl() called on non-fn_decl node"),
        }
    }

    pub fn param_decl(&self) -> &RefCell<NodeParamDecl> {
        match self {
            NodeData::ParamDecl(param_decl) => param_decl,
            _ => panic!("NodeData::param_decl() called on non-param_decl node"),
        }
    }

    pub fn type_(&self) -> &RefCell<NodeType> {
        match self {
            NodeData::Type(type_) => type_,
            _ => panic!("NodeData::type_() called on non-type_ node"),
        }
    }

    pub fn block(&self) -> &RefCell<NodeBlock> {
        match self {
            NodeData::Block(block) => block,
            _ => panic!("NodeData::block() called on non-block node"),
        }
    }

    pub fn import(&self) -> &RefCell<NodeImport> {
        match self {
            NodeData::Import(import) => import,
            _ => panic!("NodeData::import() called on non-import node"),
        }
    }

    pub fn return_(&self) -> &RefCell<NodeReturn> {
        match self {
            NodeData::Return(return_) => return_,
            _ => panic!("NodeData::return_() called on non-return_ node"),
        }
    }

    pub fn var_decl(&self) -> &RefCell<NodeVarDecl> {
        match self {
            NodeData::VarDecl(var_decl) => var_decl,
            _ => panic!("NodeData::var_decl() called on non-var_decl node"),
        }
    }

    pub fn bin_op_expr(&self) -> &RefCell<NodeBinOpExpr> {
        match self {
            NodeData::BinOpExpr(bin_op_expr) => bin_op_expr,
            _ => panic!("NodeData::bin_op_expr() called on non-bin_op_expr node"),
        }
    }

    pub fn unary_op_expr(&self) -> &RefCell<NodeUnaryOpExpr> {
        match self {
            NodeData::UnaryOpExpr(unary_op_expr) => unary_op_expr,
            _ => panic!("NodeData::unary_op_expr() called on non-unary_op_expr node"),
        }
    }

    pub fn call_expr(&self) -> &RefCell<NodeCallExpr> {
        match self {
            NodeData::CallExpr(call_expr) => call_expr,
            _ => panic!("NodeData::call_expr() called on non-call_expr node"),
        }
    }

    pub fn ident(&self) -> &RefCell<String> {
        match self {
            NodeData::Ident(ident) => ident,
            _ => panic!("NodeData::ident() called on non-ident node"),
        }
    }

    pub fn array_access_expr(&self) -> &RefCell<NodeArrayAccessExpr> {
        match self {
            NodeData::ArrayAccessExpr(array_access_expr) => array_access_expr,
            _ => panic!("NodeData::array_access_expr() called on non-array_access_expr node"),
        }
    }

    pub fn cast_expr(&self) -> &RefCell<CastExpr> {
        match self {
            NodeData::CastExpr(cast_expr) => cast_expr,
            _ => panic!("NodeData::cast_expr() called on non-cast_expr node"),
        }
    }

    pub fn if_expr(&self) -> &RefCell<NodeIfExpr> {
        match self {
            NodeData::IfExpr(if_expr) => if_expr,
            _ => panic!("NodeData::if_expr() called on non-if_expr node"),
        }
    }

    pub fn asm_expr(&self) -> &RefCell<AsmExpr> {
        match self {
            NodeData::AsmExpr(asm_expr) => asm_expr,
            _ => panic!("NodeData::asm_expr() called on non-asm_expr node"),
        }
    }

    pub fn str_lit(&self) -> &RefCell<String> {
        match self {
            NodeData::StrLit(str_lit) => str_lit,
            _ => panic!("NodeData::str_lit() called on non-str_lit node"),
        }
    }

    pub fn num_lit(&self) -> &RefCell<String> {
        match self {
            NodeData::NumLit(num_lit) => num_lit,
            _ => panic!("NodeData::num_lit() called on non-num_lit node"),
        }
    }

    pub fn bool_lit(&self) -> &RefCell<bool> {
        match self {
            NodeData::BoolLit(bool_lit) => bool_lit,
            _ => panic!("NodeData::bool_lit() called on non-bool_lit node"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub kind: RefCell<NodeKind>,
    pub line: RefCell<usize>,
    pub col: RefCell<usize>,
    pub data: RefCell<Box<NodeData>>,
    pub owner: RefCell<*mut ImportTableEntry>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Node {
        Node {
            kind: RefCell::new(kind),
            line: RefCell::new(0),
            col: RefCell::new(0),
            data: RefCell::new(Box::new(NodeData::None)),
            owner: RefCell::new(std::ptr::null_mut()),
        }
    }

    pub fn set_data(&self, data: NodeData) {
        *self.data.borrow_mut() = Box::new(data);
    }

    pub fn set_owner(&self, owner: *mut ImportTableEntry) {
        *self.owner.borrow_mut() = owner;
    }

    pub fn set_line(&self, line: usize) {
        *self.line.borrow_mut() = line;
    }

    pub fn set_col(&self, col: usize) {
        *self.col.borrow_mut() = col;
    }

    pub fn set_kind(&self, kind: NodeKind) {
        *self.kind.borrow_mut() = kind;
    }

    pub fn kind(&self) -> NodeKind {
        *self.kind.borrow()
    }

    pub fn data(&self) -> NodeData {
        *self.data.borrow().clone()
    }

    pub fn print_ast(&self, indent: usize) {
        for _ in 0..indent {
            print!(" ");
        }

        match self.kind() {
            NodeKind::Root => {
                println!("{}", self.kind().to_str());
                for child in self.data().root().borrow().children.borrow().to_owned() {
                    child.borrow().print_ast(indent + 2);
                }
            }
            NodeKind::FnProto => {
                println!(
                    "{} '{}'",
                    self.kind().to_str(),
                    self.data().fn_proto().borrow().name.borrow()
                );
                for child in self.data().fn_proto().borrow().params.borrow().clone() {
                    child.borrow().print_ast(indent + 2);
                }
                self.data()
                    .fn_proto()
                    .borrow()
                    .ret_type
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::FnDef => {
                println!("{}", self.kind().to_str());
                self.data()
                    .fn_def()
                    .borrow()
                    .proto
                    .borrow()
                    .print_ast(indent + 2);
                self.data()
                    .fn_def()
                    .borrow()
                    .body
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::FnDecl => {
                println!("{}", self.kind().to_str());
                self.data()
                    .fn_decl()
                    .borrow()
                    .proto
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::ParamDecl => {
                println!(
                    "{} '{}'",
                    self.kind().to_str(),
                    self.data().param_decl().borrow().name.borrow()
                );
                self.data()
                    .param_decl()
                    .borrow()
                    .param_type
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::Type => match self.data().type_().borrow().kind.borrow().clone() {
                TypeKind::Primitive => {
                    println!(
                        "{} '{}'",
                        self.kind().to_str(),
                        self.data().type_().borrow().name.borrow()
                    );
                }
                TypeKind::Pointer => {
                    let const_or_mut = match self.data().type_().borrow().is_const.borrow().clone()
                    {
                        true => "const",
                        false => "mut",
                    };
                    println!("'{}' ptr", const_or_mut);
                    self.data()
                        .type_()
                        .borrow()
                        .child_type
                        .clone()
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .print_ast(indent + 2);
                }
                TypeKind::Array => {
                    println!("{}", self.kind().to_str());
                    self.data()
                        .type_()
                        .borrow()
                        .child_type
                        .clone()
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .print_ast(indent + 2);
                    self.data()
                        .type_()
                        .borrow()
                        .array_size
                        .clone()
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .print_ast(indent + 2);
                }
            },
            NodeKind::Block => {
                println!("{}", self.kind().to_str());
                for stmt in self.data().block().borrow().children.borrow().clone() {
                    stmt.borrow().print_ast(indent + 2);
                }
            }
            NodeKind::Import => {
                println!(
                    "{} '{}'",
                    self.kind().to_str(),
                    self.data().import().borrow().path.borrow()
                );
            }
            NodeKind::Return => {
                println!("{}", self.kind().to_str());
                if self.data().return_().borrow().expr.borrow().is_some() {
                    self.data()
                        .return_()
                        .borrow()
                        .expr
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .print_ast(indent + 2);
                }
            }
            NodeKind::VarDecl => {
                println!(
                    "{} '{}'",
                    self.kind().to_str(),
                    self.data().var_decl().borrow().name.borrow()
                );
                if self.data().var_decl().borrow().var_type.borrow().is_some() {
                    self.data()
                        .var_decl()
                        .borrow()
                        .var_type
                        .clone()
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .print_ast(indent + 2);
                }
                if self.data().var_decl().borrow().expr.borrow().is_some() {
                    self.data()
                        .var_decl()
                        .borrow()
                        .expr
                        .clone()
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .print_ast(indent + 2);
                }
            }
            NodeKind::BinOpExpr => {
                println!(
                    "{} {}",
                    self.kind().to_str(),
                    self.data().bin_op_expr().borrow().op.borrow().to_str()
                );
                self.data()
                    .bin_op_expr()
                    .borrow()
                    .lhs
                    .borrow()
                    .print_ast(indent + 2);
                self.data()
                    .bin_op_expr()
                    .borrow()
                    .rhs
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::UnaryOpExpr => {
                println!(
                    "{} {}",
                    self.kind().to_str(),
                    self.data().unary_op_expr().borrow().op.borrow().to_str()
                );
                self.data()
                    .unary_op_expr()
                    .borrow()
                    .expr
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::CallExpr => {
                println!("{}", self.kind().to_str());
                self.data()
                    .call_expr()
                    .borrow()
                    .callee
                    .borrow()
                    .print_ast(indent + 2);
                for arg in self.data().call_expr().borrow().args.borrow().to_vec() {
                    arg.borrow().print_ast(indent + 2);
                }
            }
            NodeKind::Ident => {
                println!("{} {}", self.kind().to_str(), self.data().ident().borrow());
            }
            NodeKind::ArrayAccessExpr => {
                println!("{}", self.kind().to_str());
                self.data()
                    .array_access_expr()
                    .borrow()
                    .array
                    .borrow()
                    .print_ast(indent + 2);
                self.data()
                    .array_access_expr()
                    .borrow()
                    .index
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::CastExpr => {
                println!("{}", self.kind().to_str());
                self.data()
                    .cast_expr()
                    .borrow()
                    .expr
                    .borrow()
                    .print_ast(indent + 2);
                if self
                    .data()
                    .cast_expr()
                    .borrow()
                    .cast_type
                    .borrow()
                    .is_some()
                {
                    self.data()
                        .cast_expr()
                        .borrow()
                        .cast_type
                        .clone()
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .print_ast(indent + 2);
                }
            }
            NodeKind::Void => {
                println!("{}", self.kind().to_str());
            }
            NodeKind::IfExpr => {
                println!("{}", self.kind().to_str());
                self.data()
                    .if_expr()
                    .borrow()
                    .cond
                    .borrow()
                    .print_ast(indent + 2);
                self.data()
                    .if_expr()
                    .borrow()
                    .then
                    .borrow()
                    .print_ast(indent + 2);
                if self.data().if_expr().borrow().else_.borrow().is_some() {
                    self.data()
                        .if_expr()
                        .borrow()
                        .else_
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .print_ast(indent + 2);
                }
            }
            NodeKind::AsmExpr => {
                println!("{}", self.kind().to_str());
            }
            NodeKind::StrLit => {
                println!(
                    "{} '{}'",
                    self.kind().to_str(),
                    self.data().str_lit().borrow()
                );
            }
            NodeKind::NumLit => {
                println!(
                    "{} {}",
                    self.kind().to_str(),
                    self.data().num_lit().borrow()
                );
            }
            NodeKind::BoolLit => {
                println!(
                    "{} {}",
                    self.kind().to_str(),
                    self.data().bool_lit().borrow()
                );
            }
        }
    }
}
