// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use super::{LLVMBool, LLVMDisposeMessage};

#[derive(Debug)]
pub enum LLVMOpaqueTargetData {}
pub type LLVMTargetDataRef = *mut LLVMOpaqueTargetData;

extern "C" {
    pub fn LLVM_InitializeAllTargetInfos();
    pub fn LLVM_InitializeAllTargets();
    pub fn LLVM_InitializeAllTargetMCs();
    pub fn LLVM_InitializeAllAsmPrinters();
    pub fn LLVM_InitializeAllAsmParsers();
    pub fn LLVM_InitializeAllDisassemblers();
    pub fn LLVM_InitializeNativeTarget() -> LLVMBool;
    pub fn LLVM_InitializeNativeAsmParser() -> LLVMBool;
    pub fn LLVM_InitializeNativeAsmPrinter() -> LLVMBool;
    pub fn LLVM_InitializeNativeDisassembler() -> LLVMBool;
}

pub fn initialize_all_targets() {
    unsafe {
        LLVM_InitializeAllTargets();
    }
}

pub fn initialize_all_target_mcs() {
    unsafe {
        LLVM_InitializeAllTargetMCs();
    }
}

pub fn initialize_all_asm_printers() {
    unsafe {
        LLVM_InitializeAllAsmPrinters();
    }
}

pub fn initialize_all_asm_parsers() {
    unsafe {
        LLVM_InitializeAllAsmParsers();
    }
}

pub fn initialize_native_target() {
    unsafe {
        LLVM_InitializeNativeTarget();
    }
}

extern "C" {
    pub fn LLVMCopyStringRepOfTargetData(target_data_ref: LLVMTargetDataRef) -> *mut libc::c_char;
}

pub fn copy_string_rep_of_target_data(target_data_ref: LLVMTargetDataRef) -> String {
    unsafe {
        let c_str = LLVMCopyStringRepOfTargetData(target_data_ref);
        let rust_str = std::ffi::CStr::from_ptr(c_str)
            .to_str()
            .unwrap()
            .to_string();
        LLVMDisposeMessage(c_str);
        rust_str
    }
}

extern "C" {
    pub fn LLVMPointerSize(target_data_ref: LLVMTargetDataRef) -> libc::c_uint;
}

pub fn pointer_size(target_data_ref: LLVMTargetDataRef) -> u32 {
    unsafe { LLVMPointerSize(target_data_ref) }
}
