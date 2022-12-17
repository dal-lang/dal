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
    pub children: Vec<RefCell<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FnProtoVisibMod {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFnProto {
    pub visib_mod: FnProtoVisibMod,
    pub name: String,
    pub params: Vec<RefCell<Node>>,
    pub ret_type: RefCell<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFnDef {
    pub proto: RefCell<Node>,
    pub body: RefCell<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeFnDecl {
    pub proto: RefCell<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeParamDecl {
    pub name: String,
    pub param_type: RefCell<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Primitive,
    Pointer,
    Array,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeType {
    pub kind: TypeKind,
    pub name: String,
    pub child_type: Option<RefCell<Node>>,
    pub array_size: Option<RefCell<Node>>,
    pub is_const: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeBlock {
    pub children: Vec<RefCell<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeImport {
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeReturn {
    pub expr: Option<RefCell<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeVarDecl {
    pub name: String,
    pub is_const: bool,
    pub var_type: Option<RefCell<Node>>,
    pub expr: Option<RefCell<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOpKind {
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
    pub op: BinOpKind,
    pub lhs: RefCell<Node>,
    pub rhs: RefCell<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpKind {
    Neg,
    Not,
    BoolNot,
}

impl UnaryOpKind {
    pub fn to_str(&self) -> &str {
        match self {
            UnaryOpKind::Neg => "-",
            UnaryOpKind::Not => "~",
            UnaryOpKind::BoolNot => "!",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeUnaryOpExpr {
    pub op: UnaryOpKind,
    pub expr: RefCell<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeCallExpr {
    pub callee: RefCell<Node>,
    pub args: Vec<RefCell<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeArrayAccessExpr {
    pub array: RefCell<Node>,
    pub index: RefCell<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExpr {
    pub expr: RefCell<Node>,
    pub cast_type: Option<RefCell<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeIfExpr {
    pub cond: RefCell<Node>,
    pub then: RefCell<Node>,
    pub else_: Option<RefCell<Node>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmOutput {
    pub symbolic_name: String,
    pub constraint: String,
    pub var_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmInput {
    pub symbolic_name: String,
    pub constraint: String,
    pub expr: RefCell<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrcPos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmTokenKind {
    Template,
    Percent,
    Var,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmToken {
    pub kind: AsmTokenKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmExpr {
    pub asm_template: String,
    pub offset_map: Vec<RefCell<SrcPos>>,
    pub token_list: Vec<RefCell<AsmToken>>,
    pub output_list: Vec<RefCell<AsmOutput>>,
    pub input_list: Vec<RefCell<AsmInput>>,
    pub clobber_list: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeData {
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
    StrLit(String),
    NumLit(String),
    BoolLit(bool),
    Ident(String),
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

    pub fn ident(&self) -> &String {
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

    pub fn str_lit(&self) -> &String {
        match self {
            NodeData::StrLit(str_lit) => str_lit,
            _ => panic!("NodeData::str_lit() called on non-str_lit node"),
        }
    }

    pub fn num_lit(&self) -> &String {
        match self {
            NodeData::NumLit(num_lit) => num_lit,
            _ => panic!("NodeData::num_lit() called on non-num_lit node"),
        }
    }

    pub fn bool_lit(&self) -> &bool {
        match self {
            NodeData::BoolLit(bool_lit) => bool_lit,
            _ => panic!("NodeData::bool_lit() called on non-bool_lit node"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub kind: NodeKind,
    pub line: usize,
    pub col: usize,
    pub data: Box<NodeData>,
    pub owner: *mut ImportTableEntry,
}

impl Node {
    pub fn print_ast(&self, indent: usize) {
        for _ in 0..indent {
            print!(" ");
        }

        match self.kind {
            NodeKind::Root => {
                println!("{}", self.kind.to_str());
                for child in self.data.root().borrow().children.clone() {
                    child.borrow().print_ast(indent + 2);
                }
            }
            NodeKind::FnProto => {
                println!(
                    "{} '{}'",
                    self.kind.to_str(),
                    self.data.fn_proto().borrow().name
                );
                for child in self.data.fn_proto().borrow().params.clone() {
                    child.borrow().print_ast(indent + 2);
                }
                self.data
                    .fn_proto()
                    .borrow()
                    .ret_type
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::FnDef => {
                println!("{}", self.kind.to_str());
                self.data
                    .fn_def()
                    .borrow()
                    .proto
                    .borrow()
                    .print_ast(indent + 2);
                self.data
                    .fn_def()
                    .borrow()
                    .body
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::FnDecl => {
                println!("{}", self.kind.to_str());
                self.data
                    .fn_decl()
                    .borrow()
                    .proto
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::ParamDecl => {
                println!(
                    "{} '{}'",
                    self.kind.to_str(),
                    self.data.param_decl().borrow().name
                );
                self.data
                    .param_decl()
                    .borrow()
                    .param_type
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::Type => match self.data.type_().borrow().kind {
                TypeKind::Primitive => {
                    println!(
                        "{} '{}'",
                        self.kind.to_str(),
                        self.data.type_().borrow().name
                    );
                }
                TypeKind::Pointer => {
                    let const_or_mut = match self.data.type_().borrow().is_const {
                        true => "const",
                        false => "mut",
                    };
                    println!("'{}' ptr", const_or_mut);
                    self.data
                        .type_()
                        .borrow()
                        .child_type
                        .clone()
                        .unwrap()
                        .borrow()
                        .print_ast(indent + 2);
                }
                TypeKind::Array => {
                    println!("{}", self.kind.to_str());
                    self.data
                        .type_()
                        .borrow()
                        .child_type
                        .clone()
                        .unwrap()
                        .borrow()
                        .print_ast(indent + 2);
                    self.data
                        .type_()
                        .borrow()
                        .array_size
                        .clone()
                        .unwrap()
                        .borrow()
                        .print_ast(indent + 2);
                }
            },
            NodeKind::Block => {
                println!("{}", self.kind.to_str());
                for stmt in self.data.block().borrow().children.clone() {
                    stmt.borrow().print_ast(indent + 2);
                }
            }
            NodeKind::Import => {
                println!(
                    "{} '{}'",
                    self.kind.to_str(),
                    self.data.import().borrow().path
                );
            }
            NodeKind::Return => {
                println!("{}", self.kind.to_str());
                if self.data.return_().borrow().expr.is_some() {
                    self.data
                        .return_()
                        .borrow()
                        .expr
                        .clone()
                        .unwrap()
                        .borrow()
                        .print_ast(indent + 2);
                }
            }
            NodeKind::VarDecl => {
                println!(
                    "{} '{}'",
                    self.kind.to_str(),
                    self.data.var_decl().borrow().name
                );
                if self.data.var_decl().borrow().var_type.is_some() {
                    self.data
                        .var_decl()
                        .borrow()
                        .var_type
                        .clone()
                        .unwrap()
                        .borrow()
                        .print_ast(indent + 2);
                }
                if self.data.var_decl().borrow().expr.is_some() {
                    self.data
                        .var_decl()
                        .borrow()
                        .expr
                        .clone()
                        .unwrap()
                        .borrow()
                        .print_ast(indent + 2);
                }
            }
            NodeKind::BinOpExpr => {
                println!(
                    "{} {}",
                    self.kind.to_str(),
                    self.data.bin_op_expr().borrow().op.to_str()
                );
                self.data
                    .bin_op_expr()
                    .borrow()
                    .lhs
                    .borrow()
                    .print_ast(indent + 2);
                self.data
                    .bin_op_expr()
                    .borrow()
                    .rhs
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::UnaryOpExpr => {
                println!(
                    "{} {}",
                    self.kind.to_str(),
                    self.data.unary_op_expr().borrow().op.to_str()
                );
                self.data
                    .unary_op_expr()
                    .borrow()
                    .expr
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::CallExpr => {
                println!("{}", self.kind.to_str());
                self.data
                    .call_expr()
                    .borrow()
                    .callee
                    .borrow()
                    .print_ast(indent + 2);
                for arg in self.data.call_expr().borrow().args.clone() {
                    arg.borrow().print_ast(indent + 2);
                }
            }
            NodeKind::Ident => {
                println!("{} {}", self.kind.to_str(), self.data.ident());
            }
            NodeKind::ArrayAccessExpr => {
                println!("{}", self.kind.to_str());
                self.data
                    .array_access_expr()
                    .borrow()
                    .array
                    .borrow()
                    .print_ast(indent + 2);
                self.data
                    .array_access_expr()
                    .borrow()
                    .index
                    .borrow()
                    .print_ast(indent + 2);
            }
            NodeKind::CastExpr => {
                println!("{}", self.kind.to_str());
                self.data
                    .cast_expr()
                    .borrow()
                    .expr
                    .borrow()
                    .print_ast(indent + 2);
                if self.data.cast_expr().borrow().cast_type.is_some() {
                    self.data
                        .cast_expr()
                        .borrow()
                        .cast_type
                        .clone()
                        .unwrap()
                        .borrow()
                        .print_ast(indent + 2);
                }
            }
            NodeKind::Void => {
                println!("{}", self.kind.to_str());
            }
            NodeKind::IfExpr => {
                println!("{}", self.kind.to_str());
                self.data
                    .if_expr()
                    .borrow()
                    .cond
                    .borrow()
                    .print_ast(indent + 2);
                self.data
                    .if_expr()
                    .borrow()
                    .then
                    .borrow()
                    .print_ast(indent + 2);
                if self.data.if_expr().borrow().else_.is_some() {
                    self.data
                        .if_expr()
                        .borrow()
                        .else_
                        .clone()
                        .unwrap()
                        .borrow()
                        .print_ast(indent + 2);
                }
            }
            NodeKind::AsmExpr => {
                println!("{}", self.kind.to_str());
            }
            NodeKind::StrLit => {
                println!("{} '{}'", self.kind.to_str(), self.data.str_lit());
            }
            NodeKind::NumLit => {
                println!("{} {}", self.kind.to_str(), self.data.num_lit());
            }
            NodeKind::BoolLit => {
                println!("{} {}", self.kind.to_str(), self.data.bool_lit());
            }
        }
    }
}
