// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use super::{validate_c_string, LLVMBool, LLVMDisposeMessage, LLVMModuleRef, LLVMTargetDataRef};

#[derive(Debug)]
pub enum LLVMOpaqueTargetMachine {}
pub type LLVMTargetMachineRef = *mut LLVMOpaqueTargetMachine;

#[derive(Debug)]
pub enum LLVMTarget {}
pub type LLVMTargetRef = *mut LLVMTarget;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LLVMCodegenOptLevel {
    LLVMCodeGenLevelNone = 0,
    LLVMCodeGenLevelLess = 1,
    LLVMCodeGenLevelDefault = 2,
    LLVMCodeGenLevelAggressive = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LLVMCodeModel {
    LLVMCodeModelDefault = 0,
    LLVMCodeModelJITDefault = 1,
    LLVMCodeModelTiny = 2,
    LLVMCodeModelSmall = 3,
    LLVMCodeModelKernel = 4,
    LLVMCodeModelMedium = 5,
    LLVMCodeModelLarge = 6,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LLVMRelocMode {
    LLVMRelocDefault = 0,
    LLVMRelocStatic = 1,
    LLVMRelocPIC = 2,
    LLVMRelocDynamicNoPic = 3,
    LLVMRelocROPI = 4,
    LLVMRelocRWPI = 5,
    LLVMRelocROPI_RWPI = 6,
}

extern "C" {
    pub fn LLVMGetDefaultTargetTriple() -> *const libc::c_char;
    pub fn LLVMSetTarget(module: LLVMModuleRef, triple: *const libc::c_char);
    pub fn LLVMGetTargetFromTriple(
        triple: *const libc::c_char,
        target: *mut LLVMTargetRef,
        error: *mut *mut libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMGetHostCPUName() -> *mut libc::c_char;
    pub fn LLVMGetHostCPUFeatures() -> *mut libc::c_char;
    pub fn LLVMCreateTargetMachine(
        target: LLVMTargetRef,
        triple: *const ::libc::c_char,
        cpu: *const ::libc::c_char,
        features: *const ::libc::c_char,
        level: LLVMCodegenOptLevel,
        reloc: LLVMRelocMode,
        code_model: LLVMCodeModel,
    ) -> LLVMTargetMachineRef;
    pub fn LLVMCreateTargetDataLayout(tm: LLVMTargetMachineRef) -> LLVMTargetDataRef;
}

pub fn get_default_target_triple() -> String {
    unsafe {
        let c_str = LLVMGetDefaultTargetTriple();
        let result =
            String::from_utf8_lossy(std::ffi::CStr::from_ptr(c_str).to_bytes()).into_owned();
        LLVMDisposeMessage(c_str as *mut libc::c_char);
        result
    }
}

pub fn set_target(module: LLVMModuleRef, triple: &str) {
    let triple = validate_c_string(triple);
    unsafe {
        LLVMSetTarget(module, triple.as_ptr() as *const libc::c_char);
    }
}

pub fn get_target_from_triple(
    triple: &str,
    target: *mut LLVMTargetRef,
    error: &mut String,
) -> bool {
    let triple = validate_c_string(triple);
    unsafe {
        let mut error_ptr: *mut libc::c_char = std::ptr::null_mut();
        let result = LLVMGetTargetFromTriple(
            triple.as_ptr() as *const libc::c_char,
            target,
            &mut error_ptr,
        );
        if !error_ptr.is_null() {
            *error = String::from_utf8_lossy(std::ffi::CStr::from_ptr(error_ptr).to_bytes())
                .into_owned();
            LLVMDisposeMessage(error_ptr as *mut libc::c_char);
        }
        result != 0
    }
}

pub fn get_host_cpu_name() -> String {
    unsafe {
        let c_str = LLVMGetHostCPUName();
        let result =
            String::from_utf8_lossy(std::ffi::CStr::from_ptr(c_str).to_bytes()).into_owned();
        LLVMDisposeMessage(c_str as *mut libc::c_char);
        result
    }
}

pub fn get_host_cpu_features() -> String {
    unsafe {
        let c_str = LLVMGetHostCPUFeatures();
        let result =
            String::from_utf8_lossy(std::ffi::CStr::from_ptr(c_str).to_bytes()).into_owned();
        LLVMDisposeMessage(c_str as *mut libc::c_char);
        result
    }
}

pub fn create_target_machine(
    target: LLVMTargetRef,
    triple: &str,
    cpu: &str,
    features: &str,
    level: LLVMCodegenOptLevel,
    reloc: LLVMRelocMode,
    code_model: LLVMCodeModel,
) -> LLVMTargetMachineRef {
    let triple = validate_c_string(triple);
    let cpu = validate_c_string(cpu);
    let features = validate_c_string(features);
    unsafe {
        LLVMCreateTargetMachine(
            target,
            triple.as_ptr() as *const libc::c_char,
            cpu.as_ptr() as *const libc::c_char,
            features.as_ptr() as *const libc::c_char,
            level,
            reloc,
            code_model,
        )
    }
}

pub fn create_target_data_layout(tm: LLVMTargetMachineRef) -> LLVMTargetDataRef {
    unsafe { LLVMCreateTargetDataLayout(tm) }
}
