// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use super::{validate_c_string, LLVMBuilderRef, LLVMModuleRef, LLVMTypeRef};

extern "C" {
    pub fn LLVMDisposeMessage(message: *mut libc::c_char);
    pub fn LLVMModuleCreateWithName(name: *const libc::c_char) -> LLVMModuleRef;
    pub fn LLVMSetDataLayout(m: LLVMModuleRef, data_layout: *const libc::c_char);
    pub fn LLVMDumpModule(m: LLVMModuleRef);
    pub fn LLVMCreateBuilder() -> LLVMBuilderRef;

    pub fn LLVMInt1Type() -> LLVMTypeRef;
    pub fn LLVMInt8Type() -> LLVMTypeRef;
    pub fn LLVMInt32Type() -> LLVMTypeRef;
    pub fn LLVMIntType(width: libc::c_uint) -> LLVMTypeRef;

    pub fn LLVMFloatType() -> LLVMTypeRef;
    pub fn LLVMPointerType(t: LLVMTypeRef, addr_space: libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMVoidType() -> LLVMTypeRef;
    pub fn LLVMArrayType(t: LLVMTypeRef, count: libc::c_uint) -> LLVMTypeRef;
}

pub fn module_create_with_name(name: &str) -> LLVMModuleRef {
    let name = validate_c_string(name);
    unsafe { LLVMModuleCreateWithName(name.as_ptr() as *const libc::c_char) }
}

pub fn set_data_layout(m: LLVMModuleRef, data_layout: &str) {
    let data_layout = validate_c_string(data_layout);
    unsafe { LLVMSetDataLayout(m, data_layout.as_ptr() as *const libc::c_char) }
}

pub fn dump_module(m: LLVMModuleRef) {
    unsafe { LLVMDumpModule(m) }
}

pub fn create_builder() -> LLVMBuilderRef {
    unsafe { LLVMCreateBuilder() }
}

pub fn int1_type() -> LLVMTypeRef {
    unsafe { LLVMInt1Type() }
}

pub fn int8_type() -> LLVMTypeRef {
    unsafe { LLVMInt8Type() }
}

pub fn int32_type() -> LLVMTypeRef {
    unsafe { LLVMInt32Type() }
}

pub fn int_type(width: usize) -> LLVMTypeRef {
    unsafe { LLVMIntType(width as libc::c_uint) }
}

pub fn float_type() -> LLVMTypeRef {
    unsafe { LLVMFloatType() }
}

pub fn pointer_type(t: LLVMTypeRef, addr_space: usize) -> LLVMTypeRef {
    unsafe { LLVMPointerType(t, addr_space as libc::c_uint) }
}

pub fn void_type() -> LLVMTypeRef {
    unsafe { LLVMVoidType() }
}

pub fn array_type(t: LLVMTypeRef, count: usize) -> LLVMTypeRef {
    unsafe { LLVMArrayType(t, count as libc::c_uint) }
}
