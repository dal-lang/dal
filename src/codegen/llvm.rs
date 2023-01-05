// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

mod target;
pub use target::*;

mod target_machine;
pub use target_machine::*;

mod core;
pub use self::core::*;

mod debuginfo;
pub use self::debuginfo::*;

#[derive(Debug)]
pub enum LLVMType {}
pub type LLVMTypeRef = *mut LLVMType;

#[derive(Debug)]
pub enum LLVMValue {}
pub type LLVMValueRef = *mut LLVMValue;

#[derive(Debug)]
pub enum LLVMModule {}
pub type LLVMModuleRef = *mut LLVMModule;

#[derive(Debug)]
pub enum LLVMBuilder {}
pub type LLVMBuilderRef = *mut LLVMBuilder;

#[derive(Debug)]
pub enum LLVMOpaqueDIBuilder {}
pub type LLVMDIBuilderRef = *mut LLVMOpaqueDIBuilder;

#[derive(Debug)]
pub enum LLVMBasicBlock {}
pub type LLVMBasicBlockRef = *mut LLVMBasicBlock;

pub type LLVMBool = libc::c_int;

#[derive(Debug)]
pub enum LLVMOpaqueMetadata {}
pub type LLVMMetadataRef = *mut LLVMOpaqueMetadata;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMCallConv {
    LLVMCCallConv = 0,
    LLVMFastCallConv = 8,
    LLVMColdCallConv = 9,
    LLVMGHCCallConv = 10,
    LLVMHiPECallConv = 11,
    LLVMWebKitJSCallConv = 12,
    LLVMAnyRegCallConv = 13,
    LLVMPreserveMostCallConv = 14,
    LLVMPreserveAllCallConv = 15,
    LLVMSwiftCallConv = 16,
    LLVMCXXFASTTLSCallConv = 17,
    LLVMX86StdcallCallConv = 64,
    LLVMX86FastcallCallConv = 65,
    LLVMARMAPCSCallConv = 66,
    LLVMARMAAPCSCallConv = 67,
    LLVMARMAAPCSVFPCallConv = 68,
    LLVMMSP430INTRCallConv = 69,
    LLVMX86ThisCallCallConv = 70,
    LLVMPTXKernelCallConv = 71,
    LLVMPTXDeviceCallConv = 72,
    LLVMSPIRFUNCCallConv = 75,
    LLVMSPIRKERNELCallConv = 76,
    LLVMIntelOCLBICallConv = 77,
    LLVMX8664SysVCallConv = 78,
    LLVMWin64CallConv = 79,
    LLVMX86VectorCallCallConv = 80,
    LLVMHHVMCallConv = 81,
    LLVMHHVMCCallConv = 82,
    LLVMX86INTRCallConv = 83,
    LLVMAVRINTRCallConv = 84,
    LLVMAVRSIGNALCallConv = 85,
    LLVMAVRBUILTINCallConv = 86,
    LLVMAMDGPUVSCallConv = 87,
    LLVMAMDGPUGSCallConv = 88,
    LLVMAMDGPUPSCallConv = 89,
    LLVMAMDGPUCSCallConv = 90,
    LLVMAMDGPUKERNELCallConv = 91,
    LLVMX86RegCallCallConv = 92,
    LLVMAMDGPUHSCallConv = 93,
    LLVMMSP430BUILTINCallConv = 94,
    LLVMAMDGPULSCallConv = 95,
    LLVMAMDGPUESCallConv = 96,
}

pub fn validate_c_string(s: &str) -> String {
    if s == "" {
        let mut s = s.to_string();
        s.push('\0');
        return s;
    }
    let mut s = s.to_string();
    if s.chars().last().unwrap() != '\0' {
        s.push('\0');
    }
    s
}
