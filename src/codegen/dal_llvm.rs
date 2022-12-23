// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMDalDIType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMDalDIBuilder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMDalDICompileUnit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMDalDIScope;
pub struct LLVMDalDIFile;
pub struct LLVMDalDILexicalBlock;
pub struct LLVMDalDISubprogram;
pub struct LLVMDalDISubroutineType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMDalDILocalVariable;
pub struct LLVMDalDILocation;
pub struct LLVMDalInsertPoint;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMPassRegistery {}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMTypeRef {}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMValueRef {}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMModuleRef {}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMBuilderRef {}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMTargetDataRef {}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMTargetMachineRef {}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LLVMBasicBlockRef {}

#[allow(improper_ctypes)]
extern "C" {
    pub fn initializeLoopStrengthReducePass(P: &LLVMPassRegistery);
    pub fn initializeLowerIntrinsicPass(P: &LLVMPassRegistery);
}

#[allow(non_snake_case)]
pub fn LLVMDalInitializeLoopStrengthReducePass(pass_manager_ref: &LLVMPassRegistery) {
    unsafe {
        initializeLoopStrengthReducePass(pass_manager_ref);
    }
}

#[allow(non_snake_case)]
pub fn LLVMDalInitializeLowerIntrinsicPass(pass_manager_ref: &LLVMPassRegistery) {
    unsafe {
        initializeLowerIntrinsicPass(pass_manager_ref);
    }
}
