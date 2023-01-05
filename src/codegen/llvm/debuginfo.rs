// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use super::{validate_c_string, LLVMBool, LLVMDIBuilderRef, LLVMMetadataRef, LLVMModuleRef};

#[repr(C)]
#[derive(Debug)]
pub enum LLVMDWARFEmissionKind {
    LLVMDWARFEmissionKindNone = 0,
    LLVMDWARFEmissionKindFull,
    LLVMDWARFEmissionKindLineTablesOnly,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub enum LLVMDWARFSourceLanguage {
    LLVMDWARFSourceLanguageC89,
    LLVMDWARFSourceLanguageC,
    LLVMDWARFSourceLanguageAda83,
    LLVMDWARFSourceLanguageC_plus_plus,
    LLVMDWARFSourceLanguageCobol74,
    LLVMDWARFSourceLanguageCobol85,
    LLVMDWARFSourceLanguageFortran77,
    LLVMDWARFSourceLanguageFortran90,
    LLVMDWARFSourceLanguagePascal83,
    LLVMDWARFSourceLanguageModula2,
    LLVMDWARFSourceLanguageJava,
    LLVMDWARFSourceLanguageC99,
    LLVMDWARFSourceLanguageAda95,
    LLVMDWARFSourceLanguageFortran95,
    LLVMDWARFSourceLanguagePLI,
    LLVMDWARFSourceLanguageObjC,
    LLVMDWARFSourceLanguageObjC_plus_plus,
    LLVMDWARFSourceLanguageUPC,
    LLVMDWARFSourceLanguageD,
    LLVMDWARFSourceLanguagePython,
    LLVMDWARFSourceLanguageOpenCL,
    LLVMDWARFSourceLanguageGo,
    LLVMDWARFSourceLanguageModula3,
    LLVMDWARFSourceLanguageHaskell,
    LLVMDWARFSourceLanguageC_plus_plus_03,
    LLVMDWARFSourceLanguageC_plus_plus_11,
    LLVMDWARFSourceLanguageOCaml,
    LLVMDWARFSourceLanguageRust,
    LLVMDWARFSourceLanguageC11,
    LLVMDWARFSourceLanguageSwift,
    LLVMDWARFSourceLanguageJulia,
    LLVMDWARFSourceLanguageDylan,
    LLVMDWARFSourceLanguageC_plus_plus_14,
    LLVMDWARFSourceLanguageFortran03,
    LLVMDWARFSourceLanguageFortran08,
    LLVMDWARFSourceLanguageRenderScript,
    LLVMDWARFSourceLanguageBLISS,
    LLVMDWARFSourceLanguageMips_Assembler,
    LLVMDWARFSourceLanguageGOOGLE_RenderScript,
    LLVMDWARFSourceLanguageBORLAND_Delphi,
}

extern "C" {
    pub fn LLVMCreateDIBuilder(m: LLVMModuleRef) -> LLVMDIBuilderRef;
    pub fn LLVMDIBuilderCreateFile(
        builder: LLVMDIBuilderRef,
        filename: *const ::libc::c_char,
        filename_len: ::libc::size_t,
        directory: *const ::libc::c_char,
        directory_len: ::libc::size_t,
    ) -> LLVMMetadataRef;
    pub fn LLVMDIBuilderCreateCompileUnit(
        builder: LLVMDIBuilderRef,
        lang: LLVMDWARFSourceLanguage,
        metadata: LLVMMetadataRef,
        producer: *const libc::c_char,
        producer_len: libc::size_t,
        is_optimized: LLVMBool,
        flags: *const libc::c_char,
        flags_len: libc::size_t,
        runtime_ver: libc::c_uint,
        split_name: *const libc::c_char,
        split_name_len: libc::size_t,
        kind: LLVMDWARFEmissionKind,
        dwoid: libc::c_uint,
        split_debug_inlining: LLVMBool,
        debug_info_for_profiling: LLVMBool,
        sysroot: *const libc::c_char,
        sysroot_len: libc::size_t,
        sdk: *const libc::c_char,
        sdk_len: libc::size_t,
    ) -> LLVMMetadataRef;
    pub fn LLVMDIBuilderGetOrCreateArray(
        builder: LLVMDIBuilderRef,
        data: *mut LLVMMetadataRef,
        len: libc::size_t,
    ) -> LLVMMetadataRef;
    pub fn LLVMDIBuilderGetOrCreateSubrange(
        builder: LLVMDIBuilderRef,
        lower_bound: i64,
        count: i64,
    ) -> LLVMMetadataRef;
    pub fn LLVMDIBuilderCreateArrayType(
        builder: LLVMDIBuilderRef,
        size: u64,
        align: u32,
        ty: LLVMMetadataRef,
        subscripts: *mut LLVMMetadataRef,
        subscripts_len: libc::c_uint,
    ) -> LLVMMetadataRef;
}

pub fn di_create_di_builder(m: LLVMModuleRef) -> LLVMDIBuilderRef {
    unsafe { LLVMCreateDIBuilder(m) }
}

pub fn di_create_di_file(
    builder: LLVMDIBuilderRef,
    filename: &str,
    directory: &str,
) -> LLVMMetadataRef {
    let filename = validate_c_string(filename);
    let directory = validate_c_string(directory);
    unsafe {
        LLVMDIBuilderCreateFile(
            builder,
            filename.as_ptr() as *const libc::c_char,
            filename.len(),
            directory.as_ptr() as *const libc::c_char,
            directory.len(),
        )
    }
}

pub fn di_create_compile_unit(
    builder: LLVMDIBuilderRef,
    lang: LLVMDWARFSourceLanguage,
    metadata: LLVMMetadataRef,
    producer: &str,
    is_optimized: bool,
    flags: &str,
    runtime_ver: u32,
    split_name: &str,
    kind: LLVMDWARFEmissionKind,
    dwoid: u32,
    split_debug_inlining: bool,
    debug_info_for_profiling: bool,
    sysroot: &str,
    sdk: &str,
) -> LLVMMetadataRef {
    let producer = validate_c_string(producer);
    let is_optimized = if is_optimized { 1 } else { 0 };
    let flags = validate_c_string(flags);
    let runtime_ver = runtime_ver as libc::c_uint;
    let split_name = validate_c_string(split_name);
    let dwoid = dwoid as libc::c_uint;
    let split_debug_inlining = if split_debug_inlining { 1 } else { 0 };
    let debug_info_for_profiling = if debug_info_for_profiling { 1 } else { 0 };
    let sysroot = validate_c_string(sysroot);
    let sdk = validate_c_string(sdk);
    unsafe {
        LLVMDIBuilderCreateCompileUnit(
            builder,
            lang,
            metadata,
            producer.as_ptr() as *const libc::c_char,
            producer.len(),
            is_optimized,
            flags.as_ptr() as *const libc::c_char,
            flags.len(),
            runtime_ver,
            split_name.as_ptr() as *const libc::c_char,
            split_name.len(),
            kind,
            dwoid,
            split_debug_inlining,
            debug_info_for_profiling,
            sysroot.as_ptr() as *const libc::c_char,
            sysroot.len(),
            sdk.as_ptr() as *const libc::c_char,
            sdk.len(),
        )
    }
}

pub fn di_get_or_create_array(
    builder: LLVMDIBuilderRef,
    data: &mut Vec<LLVMMetadataRef>,
) -> LLVMMetadataRef {
    unsafe { LLVMDIBuilderGetOrCreateArray(builder, data.as_mut_ptr(), data.len() as libc::size_t) }
}

pub fn di_get_or_create_subrange(
    builder: LLVMDIBuilderRef,
    lower_bound: i64,
    count: i64,
) -> LLVMMetadataRef {
    unsafe { LLVMDIBuilderGetOrCreateSubrange(builder, lower_bound, count) }
}

pub fn di_create_array_type(
    builder: LLVMDIBuilderRef,
    size: u64,
    align: u32,
    ty: LLVMMetadataRef,
) -> LLVMMetadataRef {
    let mut subranges = Vec::new();
    di_get_or_create_subrange(builder, 0, size as i64 - 1);
    di_get_or_create_array(builder, &mut subranges);
    unsafe {
        LLVMDIBuilderCreateArrayType(
            builder,
            size,
            align,
            ty,
            subranges.as_mut_ptr(),
            subranges.len() as libc::c_uint,
        )
    }
}
