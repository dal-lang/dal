// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use std::{cell::RefCell, collections::HashMap};

use crate::{compiler::ast::Node, ErrColor, ErrMsg};

use super::dal_llvm::{
    LLVMBasicBlockRef, LLVMBuilderRef, LLVMDalDIBuilder, LLVMDalDICompileUnit,
    LLVMDalDILocalVariable, LLVMDalDIScope, LLVMDalDIType, LLVMModuleRef, LLVMTargetDataRef,
    LLVMTargetMachineRef, LLVMTypeRef, LLVMValueRef,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeTableEntryPtr {
    pub ptr: *mut TypeTableEntry,
    pub ptr_is_const: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeTableEntryInt {
    pub is_signed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeTableEntryArray {
    pub child_type: *mut TypeTableEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeTableEntryKind {
    Invalid,
    Void,
    Bool,
    Int,
    Float,
    Ptr,
    Array,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeTableEntryData {
    Ptr(RefCell<TypeTableEntryPtr>),
    Int(RefCell<TypeTableEntryInt>),
    Array(RefCell<TypeTableEntryArray>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeTableEntry {
    pub kind: TypeTableEntryKind,
    pub type_ref: LLVMTypeRef,
    pub di_type: LLVMDalDIType,
    pub size_in_bits: u64,
    pub align_in_bits: u64,
    pub name: String,
    pub data: Box<TypeTableEntryData>,
    pub ptr_const_parent: *mut TypeTableEntry,
    pub ptr_mut_parent: *mut TypeTableEntry,
    pub array_by_size: HashMap<isize, *mut TypeTableEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportTableEntry {
    pub root: *mut Node,
    pub path: String,
    pub src_code: String,
    pub line_offsets: Vec<usize>,
}

impl ImportTableEntry {
    pub fn new(path: &str, src_code: &str, line_offsets: Vec<usize>) -> Self {
        Self {
            root: std::ptr::null_mut(),
            path: path.to_string(),
            src_code: src_code.to_string(),
            line_offsets,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnTableEntry {
    pub fn_value: LLVMValueRef,
    pub fn_proto: *mut Node,
    pub fn_def: *mut Node,
    pub is_extern: bool,
    pub internal_linkage: bool,
    pub call_conv: u32,
    pub import_entry: *mut ImportTableEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinTypes {
    pub entey_bool: *mut TypeTableEntry,
    pub entry_u8: *mut TypeTableEntry,
    pub entry_i32: *mut TypeTableEntry,
    pub entry_isize: *mut TypeTableEntry,
    pub entry_f32: *mut TypeTableEntry,
    pub entry_str_lit: *mut TypeTableEntry,
    pub entry_void: *mut TypeTableEntry,
    pub entry_invalid: *mut TypeTableEntry,
    pub entry_no_return: *mut TypeTableEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeGen {
    pub module: LLVMModuleRef,
    pub errs: Vec<ErrMsg>,
    pub builder: LLVMBuilderRef,
    pub dbuilder: LLVMDalDIBuilder,
    pub compile_unit: LLVMDalDICompileUnit,
    pub lib_search_paths: Vec<String>,
    pub fn_table: HashMap<String, *mut FnTableEntry>,
    pub str_table: HashMap<String, LLVMValueRef>,
    pub type_table: HashMap<String, *mut TypeTableEntry>,
    pub link_table: HashMap<String, bool>,
    pub import_table: HashMap<String, *mut ImportTableEntry>,
    pub builtin_types: BuiltinTypes,
    pub target_data_ref: LLVMTargetDataRef,
    pub ptr_size_bytes: u32,
    pub is_static: bool,
    pub strip_debug_symbol: bool,
    pub include_bootstrap: bool,
    // pub build_type: CodeGenBuildType,
    pub target_machine_ref: LLVMTargetMachineRef,
    pub is_native_target: bool,
    pub src_dir: String,
    pub root_out_name: String,
    pub fn_defs: Vec<*mut FnTableEntry>,
    pub fn_protos: Vec<*mut FnTableEntry>,
    // pub out_type: OutType,
    pub cur_fn: *mut FnTableEntry,
    pub cur_basic_block: LLVMBasicBlockRef,
    pub cur_block_context: *mut BlockContext,
    pub c_std_int_used: bool,
    pub version_major: u32,
    pub version_minor: u32,
    pub version_patch: u32,
    pub verbose: bool,
    pub err_color: ErrColor,
    pub root_import: *mut ImportTableEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVariableTableEntry {
    pub name: String,
    pub type_: *mut TypeTableEntry,
    pub value_ref: LLVMValueRef,
    pub is_const: bool,
    pub is_ptr: bool,
    pub decl_node: *mut Node,
    pub di_loc_var: LLVMDalDILocalVariable,
    pub di_scope: LLVMDalDIScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockContext {
    pub node: *mut Node,
    pub root: *mut BlockContext,
    pub parent: *mut BlockContext,
    pub variable_table: HashMap<String, *mut LocalVariableTableEntry>,
    pub di_scope: LLVMDalDIScope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeNode {
    pub entry: *mut TypeTableEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnProtoNode {
    pub entry: *mut FnTableEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnDefNode {
    pub implicit_return_type: *mut TypeTableEntry,
    pub block_context: *mut BlockContext,
    pub skip: bool,
    pub all_block_contexts: Vec<*mut BlockContext>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprNode {
    pub type_entry: *mut TypeTableEntry,
    pub block_context: *mut BlockContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignNode {
    pub var_entry: *mut LocalVariableTableEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockNode {
    pub block_context: *mut BlockContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeGenNodeData {
    TypeNode(RefCell<TypeNode>),
    FnDefNode(RefCell<FnDefNode>),
    FnProtoNode(RefCell<FnProtoNode>),
    AssignNode(RefCell<AssignNode>),
    BlockNode(RefCell<BlockNode>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeGenNode {
    pub data: CodeGenNodeData,
    pub expr_node: *mut ExprNode,
}
