// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use std::{collections::HashMap, ptr::null_mut};

use crate::{
    ast::{FnProtoVisibMod, Node, NodeKind, TypeNodeKind},
    dal_pointer::*,
    err_msg::{ErrColor, ErrMsg},
};

use super::{dal_llvm, llvm, CodeGenBuildType, OutType};

#[derive(Clone)]
pub struct TypeTableEntryPointer {
    pub child: DalPtr<TypeTableEntry>,
    pub is_const: bool,
}

impl TypeTableEntryPointer {
    pub fn new() -> Self {
        Self {
            child: DalPtr::null(),
            is_const: false,
        }
    }
}

#[derive(Clone)]
pub struct TypeTableEntryInt {
    pub is_signed: bool,
}

impl TypeTableEntryInt {
    pub fn new() -> Self {
        Self { is_signed: false }
    }
}

#[derive(Clone)]
pub struct TypeTableEntryArray {
    pub child: DalWeakPtr<TypeTableEntry>,
}

impl TypeTableEntryArray {
    pub fn new() -> Self {
        Self {
            child: DalWeakPtr::new(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum TypeTableEntryKind {
    Invalid,
    Void,
    Bool,
    Int,
    Float,
    Pointer,
    Array,
}

#[derive(Clone)]
pub enum TypeTableEntryData {
    Pointer(TypeTableEntryPointer),
    Int(TypeTableEntryInt),
    Array(TypeTableEntryArray),
    Invalid,
}

impl TypeTableEntryData {
    pub fn pointer(&mut self) -> &mut TypeTableEntryPointer {
        match self {
            Self::Pointer(p) => p,
            _ => panic!("TypeTableEntryData::pointer() called on non-pointer"),
        }
    }

    pub fn int(&mut self) -> &mut TypeTableEntryInt {
        match self {
            Self::Int(i) => i,
            _ => panic!("TypeTableEntryData::int() called on non-int"),
        }
    }

    pub fn array(&mut self) -> &mut TypeTableEntryArray {
        match self {
            Self::Array(a) => a,
            _ => panic!("TypeTableEntryData::array() called on non-array"),
        }
    }
}

#[derive(Clone)]
pub struct TypeTableEntry {
    pub kind: TypeTableEntryKind,
    pub type_ref: llvm::LLVMTypeRef,
    pub di_type: llvm::LLVMMetadataRef,
    pub size_in_bits: usize,
    pub align_in_bits: usize,
    pub name: String,
    pub data: TypeTableEntryData,
    pub pointer_const_parent: DalWeakPtr<TypeTableEntry>,
    pub pointer_mut_parent: DalWeakPtr<TypeTableEntry>,
    pub array_by_size: HashMap<usize, DalWeakPtr<TypeTableEntry>>,
}

impl TypeTableEntry {
    pub fn new(kind: TypeTableEntryKind) -> Self {
        let data: TypeTableEntryData = match kind {
            TypeTableEntryKind::Pointer => {
                TypeTableEntryData::Pointer(TypeTableEntryPointer::new())
            }
            TypeTableEntryKind::Int => TypeTableEntryData::Int(TypeTableEntryInt::new()),
            TypeTableEntryKind::Array => TypeTableEntryData::Array(TypeTableEntryArray::new()),
            _ => TypeTableEntryData::Invalid,
        };
        Self {
            kind,
            type_ref: null_mut(),
            di_type: null_mut(),
            size_in_bits: 0,
            align_in_bits: 0,
            name: String::new(),
            data,
            pointer_const_parent: DalWeakPtr::new(),
            pointer_mut_parent: DalWeakPtr::new(),
            array_by_size: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct ImportTableEntry {
    pub root: DalPtr<Node>,
    pub path: String,
    pub di_file: llvm::LLVMMetadataRef,
    pub src: String,
    pub line_offsets: Vec<usize>,
    pub fn_table: HashMap<String, DalPtr<FnTableEntry>>,
}

impl ImportTableEntry {
    pub fn new() -> Self {
        Self {
            root: DalPtr::null(),
            path: String::new(),
            di_file: null_mut(),
            src: String::new(),
            line_offsets: Vec::new(),
            fn_table: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct FnTableEntry {
    pub fn_value: llvm::LLVMValueRef,
    pub proto_node: DalPtr<Node>,
    pub fn_def_node: DalPtr<Node>,
    pub is_extern: bool,
    pub internal_linkage: bool,
    pub calling_convention: llvm::LLVMCallConv,
    pub import_table_entry: DalPtr<ImportTableEntry>,
}

impl FnTableEntry {
    pub fn new() -> Self {
        Self {
            fn_value: null_mut(),
            proto_node: DalPtr::null(),
            fn_def_node: DalPtr::null(),
            is_extern: false,
            internal_linkage: false,
            calling_convention: llvm::LLVMCallConv::LLVMCCallConv,
            import_table_entry: DalPtr::null(),
        }
    }
}

#[derive(Clone)]
pub struct BuiltinTypes {
    pub entry_bool: DalPtr<TypeTableEntry>,
    pub entry_u8: DalPtr<TypeTableEntry>,
    pub entry_i32: DalPtr<TypeTableEntry>,
    pub entry_isize: DalPtr<TypeTableEntry>,
    pub entry_f32: DalPtr<TypeTableEntry>,
    pub entry_string_literal: DalPtr<TypeTableEntry>,
    pub entry_void: DalPtr<TypeTableEntry>,
    pub entry_invalid: DalPtr<TypeTableEntry>,
}

impl BuiltinTypes {
    pub fn new() -> Self {
        Self {
            entry_bool: DalPtr::null(),
            entry_u8: DalPtr::null(),
            entry_i32: DalPtr::null(),
            entry_isize: DalPtr::null(),
            entry_f32: DalPtr::null(),
            entry_string_literal: DalPtr::null(),
            entry_void: DalPtr::null(),
            entry_invalid: DalPtr::null(),
        }
    }
}

#[derive(Clone)]
pub struct LocalVariableTableEntry {
    pub name: String,
    pub type_entry: *mut TypeTableEntry,
    pub value_ref: llvm::LLVMValueRef,
    pub is_const: bool,
    pub is_ptr: bool,
    pub decl_node: *mut Node,
    pub di_local_var: *mut dal_llvm::LLVMDILocalVariable,
    pub arg_index: isize,
}

#[derive(Clone)]
pub struct BlockContext {
    pub node: *mut Node,
    pub root: *mut BlockContext,
    pub parent: *mut BlockContext,
    pub var_table: HashMap<String, *mut LocalVariableTableEntry>,
    pub di_scope: *mut dal_llvm::LLVMDIScope,
}

#[derive(Clone)]
pub struct TypeNode {
    pub type_table_entry: DalPtr<TypeTableEntry>,
}

impl TypeNode {
    pub fn new() -> Self {
        Self {
            type_table_entry: DalPtr::null(),
        }
    }
}

#[derive(Clone)]
pub struct FnDefNode {
    pub implicit_return_type: *mut TypeTableEntry,
    pub block_context: *mut BlockContext,
    pub skip: bool,
    pub all_blocks: Vec<*mut BlockContext>,
}

impl FnDefNode {
    pub fn new() -> Self {
        Self {
            implicit_return_type: null_mut(),
            block_context: null_mut(),
            skip: false,
            all_blocks: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct FnProtoNode {
    pub fn_table_entry: *mut FnTableEntry,
}

impl FnProtoNode {
    pub fn new() -> Self {
        Self {
            fn_table_entry: null_mut(),
        }
    }
}

#[derive(Clone)]
pub struct AssignNode {
    pub var_entry: *mut LocalVariableTableEntry,
}

impl AssignNode {
    pub fn new() -> Self {
        Self {
            var_entry: null_mut(),
        }
    }
}

#[derive(Clone)]
pub struct BlockNode {
    pub block_context: *mut BlockContext,
}

impl BlockNode {
    pub fn new() -> Self {
        Self {
            block_context: null_mut(),
        }
    }
}

#[derive(Clone)]
pub struct ExprNode {
    pub type_entry: *mut TypeTableEntry,
    pub block_context: *mut BlockContext,
}

impl ExprNode {
    pub fn new() -> Self {
        Self {
            type_entry: null_mut(),
            block_context: null_mut(),
        }
    }
}

#[derive(Clone)]
pub enum CodeGenNodeData {
    None,
    Type(DalPtr<TypeNode>),
    FnDef(DalPtr<FnDefNode>),
    FnProtoNode(DalPtr<FnProtoNode>),
    AssignNode(DalPtr<AssignNode>),
    BlockNode(DalPtr<BlockNode>),
}

impl CodeGenNodeData {
    pub fn new() -> Self {
        Self::None
    }

    pub fn type_node(&self) -> &DalPtr<TypeNode> {
        match self {
            Self::Type(node) => node,
            _ => panic!("Invalid node type"),
        }
    }

    pub fn fn_def_node(&self) -> &DalPtr<FnDefNode> {
        match self {
            Self::FnDef(node) => node,
            _ => panic!("Invalid node type"),
        }
    }

    pub fn fn_proto_node(&self) -> &DalPtr<FnProtoNode> {
        match self {
            Self::FnProtoNode(node) => node,
            _ => panic!("Invalid node type"),
        }
    }

    pub fn assign_node(&self) -> &DalPtr<AssignNode> {
        match self {
            Self::AssignNode(node) => node,
            _ => panic!("Invalid node type"),
        }
    }

    pub fn block_node(&self) -> &DalPtr<BlockNode> {
        match self {
            Self::BlockNode(node) => node,
            _ => panic!("Invalid node type"),
        }
    }
}

#[derive(Clone)]
pub struct CodeGenNode {
    pub data: CodeGenNodeData,
    pub expr_node: ExprNode,
}

impl CodeGenNode {
    pub fn new() -> Self {
        Self {
            data: CodeGenNodeData::new(),
            expr_node: ExprNode::new(),
        }
    }
}

pub struct CodeGen {
    pub module: llvm::LLVMModuleRef,
    pub errors: Vec<ErrMsg>,
    pub builder: llvm::LLVMBuilderRef,
    pub dbuilder: llvm::LLVMDIBuilderRef,
    pub compile_unit: llvm::LLVMMetadataRef,
    pub lib_search_paths: Vec<String>,
    pub fn_table: HashMap<String, DalPtr<FnTableEntry>>,
    pub str_table: HashMap<String, llvm::LLVMValueRef>,
    pub type_table: HashMap<String, DalPtr<TypeTableEntry>>,
    pub link_table: HashMap<String, bool>,
    pub import_table: DalPtr<HashMap<String, DalPtr<ImportTableEntry>>>,
    pub builtin_types: BuiltinTypes,
    pub target_data_ref: llvm::LLVMTargetDataRef,
    pub pointer_size_bytes: u64,
    pub is_static: bool,
    pub strip_debug_symbols: bool,
    pub insert_bootstrap_code: bool,
    pub build_type: CodeGenBuildType,
    pub target_machine: llvm::LLVMTargetMachineRef,
    pub is_native_target: bool,
    pub root_source_dir: String,
    pub root_out_name: String,
    pub fn_defs: Vec<DalPtr<FnTableEntry>>,
    pub fn_protos: Vec<DalPtr<FnTableEntry>>,
    pub out_type: OutType,
    pub cur_fn: DalPtr<FnTableEntry>,
    pub cur_basic_block: llvm::LLVMBasicBlockRef,
    pub cur_block_context: DalPtr<BlockContext>,
    pub verbose: bool,
    pub err_color: ErrColor,
    pub root_import: DalWeakPtr<ImportTableEntry>,
}

fn parse_int(number: &str) -> usize {
    usize::from_str_radix(number, 10).unwrap()
}

impl CodeGen {
    pub(super) fn add_node_error(&mut self, node: &Node, msg: String) {
        let mut err_msg = ErrMsg::new();
        err_msg.line_start = node.line;
        err_msg.col_start = node.col;
        err_msg.line_end = 0;
        err_msg.col_end = 0;
        err_msg.msg = msg;
        err_msg.path = node.owner().borrow().path.clone();
        err_msg.src = node.owner().borrow().src.clone();
        err_msg.line_offsets = node.owner().borrow().line_offsets.clone();
        self.errors.push(err_msg);
    }

    fn get_array_type(
        &mut self,
        child_ty: DalPtr<TypeTableEntry>,
        size: usize,
    ) -> DalPtr<TypeTableEntry> {
        let existing_entry: Option<DalWeakPtr<TypeTableEntry>>;
        {
            let v = child_ty.borrow();
            if let Some(v) = v.array_by_size.get(&size) {
                existing_entry = Some(v.clone());
            } else {
                existing_entry = None;
            }
        }
        if existing_entry.is_some() {
            return existing_entry.unwrap().upgrade().unwrap();
        } else {
            let entry = DalPtr::new(TypeTableEntry::new(TypeTableEntryKind::Array));
            {
                let mut et = entry.borrow_mut();
                et.type_ref = llvm::array_type(child_ty.borrow().type_ref, size);
                et.name = format!("[{}; {}]", child_ty.borrow().name, size);
                et.size_in_bits = child_ty.borrow().size_in_bits * size;
                et.align_in_bits = child_ty.borrow().align_in_bits;
                et.di_type = llvm::di_create_array_type(
                    self.dbuilder,
                    et.size_in_bits as u64,
                    et.align_in_bits as u32,
                    child_ty.borrow().di_type,
                );
                et.data.array().child = child_ty.downgrade();
            }
            self.type_table
                .insert(entry.borrow().name.clone(), entry.clone());
            child_ty
                .borrow_mut()
                .array_by_size
                .insert(size, entry.downgrade());
            entry
        }
    }

    fn resolve_type(&mut self, node: &mut Node) -> DalPtr<TypeTableEntry> {
        assert_eq!(node.kind, NodeKind::Type);
        assert!(node.codegen_node.is_null());
        let codegen_node = DalPtr::new(CodeGenNode::new());
        {
            codegen_node.borrow_mut().data = CodeGenNodeData::Type(DalPtr::new(TypeNode::new()));
        }

        node.codegen_node = codegen_node.clone();

        let x = codegen_node.borrow();
        let mut type_node = x.data.type_node().borrow_mut();
        match node.data.type_().kind {
            TypeNodeKind::Primitive => {
                let name = &node.data.type_().name;
                let table_entry = self.type_table.get(name);
                if let Some(entry) = table_entry {
                    type_node.type_table_entry = entry.clone();
                } else {
                    self.add_node_error(node, format!("Unknown type '{}'", name));
                    type_node.type_table_entry = self.builtin_types.entry_invalid.clone();
                }
                return type_node.type_table_entry.clone();
            }
            TypeNodeKind::Pointer => {
                self.resolve_type(&mut node.data.type_().ty.borrow_mut());
                let child_ty = node
                    .data
                    .type_()
                    .ty
                    .borrow()
                    .codegen_node
                    .borrow()
                    .data
                    .type_node()
                    .borrow()
                    .type_table_entry
                    .clone();
                assert!(child_ty.is_some());
                if child_ty.borrow().kind == TypeTableEntryKind::Invalid {
                    return child_ty;
                }
                type_node.type_table_entry =
                    self.get_pointer_to_type(child_ty, node.data.type_().is_const);
                return type_node.type_table_entry.clone();
            }
            TypeNodeKind::Array => {
                self.resolve_type(&mut node.data.type_().ty.borrow_mut());
                let child_ty = node
                    .data
                    .type_()
                    .ty
                    .borrow()
                    .codegen_node
                    .borrow()
                    .data
                    .type_node()
                    .borrow()
                    .type_table_entry
                    .clone();

                let size_node = node.data.type_().size.clone();
                let mut size = 0;
                if size_node.borrow().kind != NodeKind::NumLit {
                    self.add_node_error(
                        &size_node.borrow(),
                        "Array size must be a constant integer literal".to_string(),
                    );
                } else {
                    size = parse_int(&size_node.borrow().data.num_lit());
                }

                type_node.type_table_entry = self.get_array_type(child_ty, size);
                type_node.type_table_entry.clone()
            }
        }
    }

    fn resolve_fn_proto(&mut self, proto: DalPtr<Node>) {
        assert_eq!(proto.borrow().kind, NodeKind::FnProto);

        for param in &proto.borrow().data.fn_proto().params {
            assert_eq!(param.borrow().kind, NodeKind::ParamDecl);

            let type_entry = self.resolve_type(&mut param.borrow().data.param_decl().ty_mut());
            if type_entry.borrow().kind == TypeTableEntryKind::Void {
                if proto.borrow().data.fn_proto().visib_mod == FnProtoVisibMod::Public {
                    self.add_node_error(
                        &param.borrow().data.param_decl().ty(),
                        "Parameter cannot be of type 'void'".to_string(),
                    );
                }
            }
        }

        self.resolve_type(&mut proto.borrow().data.fn_proto_mut().ret_type);
    }

    fn preview_fn_decl(&mut self, import_table: DalPtr<ImportTableEntry>, node: DalPtr<Node>) {
        match node.borrow().kind {
            NodeKind::ExternBlock => {
                for fn_decl in node.borrow().data.extern_().fn_decls.iter() {
                    let fn_decl = fn_decl.borrow();
                    assert_eq!(fn_decl.kind, NodeKind::FnDecl);
                    let proto = fn_decl.data.fn_decl().proto.clone();
                    let is_public =
                        proto.borrow().data.fn_proto().visib_mod == FnProtoVisibMod::Public;

                    let fn_table_entry = DalPtr::new(FnTableEntry::new());
                    {
                        let mut entry = fn_table_entry.borrow_mut();
                        entry.proto_node = proto.clone();
                        entry.is_extern = true;
                        entry.calling_convention = llvm::LLVMCallConv::LLVMCCallConv;
                        entry.import_table_entry = import_table.clone();
                    }

                    let name = proto.borrow().data.fn_proto().name.clone();
                    self.fn_protos.push(fn_table_entry.clone());
                    import_table
                        .borrow_mut()
                        .fn_table
                        .insert(name.clone(), fn_table_entry.clone());
                    if is_public {
                        self.fn_table.insert(name.clone(), fn_table_entry.clone());
                    }

                    self.resolve_fn_proto(proto);
                }
            }
            _ => {}
        }
    }

    fn find_fn_decl_root(&mut self, import_table: DalPtr<ImportTableEntry>, root: DalPtr<Node>) {
        assert_eq!(root.borrow().kind, NodeKind::Root);

        for child in &root.borrow().data.root().children {
            self.preview_fn_decl(import_table.clone(), child.clone());
        }
    }

    pub(super) fn semantic_analyze(&mut self) {
        {
            let it = self.import_table.clone();
            for (_, entry) in it.borrow().iter() {
                if entry.is_some() {
                    self.find_fn_decl_root(entry.clone(), entry.borrow().root.clone());
                } else {
                    break;
                }
            }
        }
    }
}
