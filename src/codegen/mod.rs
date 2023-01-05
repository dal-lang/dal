// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

pub mod dal_llvm;
pub mod llvm;

mod analyze;
pub use analyze::*;

use crate::{
    ast::{FnProtoVisibMod, NodeKind},
    dal_pointer::{DalPtr, DalWeakPtr},
    err_msg::{ErrColor, ErrMsg},
    lexer::{print_tokens, tokenize, Tokenization},
    parser::parse,
    DAL_STD_DIR, DAL_VERSION,
};
use std::{collections::HashMap, path::Path, ptr::null_mut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutType {
    Exe,
    Lib,
    Obj,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeGenBuildType {
    Debug,
    Release,
}

impl CodeGen {
    pub fn new(root_source_dir: String) -> Self {
        Self {
            module: null_mut(),
            errors: Vec::new(),
            builder: null_mut(),
            dbuilder: null_mut(),
            compile_unit: null_mut(),
            lib_search_paths: Vec::new(),
            fn_table: HashMap::new(),
            str_table: HashMap::new(),
            type_table: HashMap::new(),
            link_table: HashMap::new(),
            import_table: DalPtr::new(HashMap::new()),
            builtin_types: BuiltinTypes::new(),
            target_data_ref: null_mut(),
            pointer_size_bytes: 0,
            is_static: false,
            strip_debug_symbols: false,
            insert_bootstrap_code: false,
            build_type: CodeGenBuildType::Debug,
            target_machine: null_mut(),
            is_native_target: false,
            root_source_dir,
            root_out_name: String::new(),
            fn_defs: Vec::new(),
            fn_protos: Vec::new(),
            out_type: OutType::Exe,
            cur_fn: DalPtr::null(),
            cur_basic_block: null_mut(),
            cur_block_context: DalPtr::null(),
            verbose: false,
            err_color: ErrColor::Auto,
            root_import: DalWeakPtr::new(),
        }
    }

    fn get_pointer_to_type(
        &mut self,
        child_ty: DalPtr<TypeTableEntry>,
        is_const: bool,
    ) -> DalPtr<TypeTableEntry> {
        let parent_ptr: Option<DalPtr<TypeTableEntry>>;
        {
            parent_ptr = child_ty.borrow().pointer_mut_parent.upgrade();
        }
        if parent_ptr.is_some() {
            parent_ptr.unwrap()
        } else {
            let entry = DalPtr::new(TypeTableEntry::new(TypeTableEntryKind::Pointer));
            {
                let mut et = entry.borrow_mut();
                et.type_ref = llvm::pointer_type(child_ty.borrow().type_ref, 0);
                et.name = format!(
                    "*{} {}",
                    if is_const { "const" } else { "mut" },
                    child_ty.borrow().name
                );
                et.size_in_bits = (self.pointer_size_bytes * 8) as usize;
                et.align_in_bits = (self.pointer_size_bytes * 8) as usize;
            }
            self.type_table
                .insert(entry.borrow().name.clone(), entry.clone());
            {
                child_ty.borrow_mut().pointer_mut_parent = self
                    .type_table
                    .get(&entry.borrow().name)
                    .unwrap()
                    .downgrade();
            }
            return entry;
        }
    }

    fn define_primitive_types(&mut self) {
        {
            let entry = DalPtr::new(TypeTableEntry::new(TypeTableEntryKind::Invalid));
            entry.borrow_mut().name = "(invalid)".to_string();
            self.builtin_types.entry_invalid = entry;
        }
        {
            let mut entry = TypeTableEntry::new(TypeTableEntryKind::Bool);
            entry.type_ref = llvm::int1_type();
            entry.name = "bool".to_string();
            entry.size_in_bits = 1;
            entry.align_in_bits = 8;
            let entry = DalPtr::new(entry);
            self.type_table
                .insert(entry.borrow().name.clone(), entry.clone());
            self.builtin_types.entry_bool = entry;
        }
        {
            let mut entry = TypeTableEntry::new(TypeTableEntryKind::Int);
            entry.type_ref = llvm::int8_type();
            entry.name = "u8".to_string();
            entry.size_in_bits = 8;
            entry.align_in_bits = 8;
            let entry = DalPtr::new(entry);
            self.type_table
                .insert(entry.borrow().name.clone(), entry.clone());
            self.builtin_types.entry_u8 = entry;
        }
        self.builtin_types.entry_string_literal =
            self.get_pointer_to_type(self.builtin_types.entry_u8.clone(), true);
        {
            let mut entry = TypeTableEntry::new(TypeTableEntryKind::Int);
            entry.type_ref = llvm::int32_type();
            entry.name = "i32".to_string();
            entry.size_in_bits = 32;
            entry.align_in_bits = 32;
            entry.data.int().is_signed = true;
            let entry = DalPtr::new(entry);
            self.type_table
                .insert(entry.borrow().name.clone(), entry.clone());
            self.builtin_types.entry_i32 = entry;
        }
        {
            let mut entry = TypeTableEntry::new(TypeTableEntryKind::Int);
            entry.type_ref = llvm::int_type(self.pointer_size_bytes as usize * 8);
            entry.name = "isize".to_string();
            entry.size_in_bits = (self.pointer_size_bytes * 8) as usize;
            entry.align_in_bits = (self.pointer_size_bytes * 8) as usize;
            entry.data.int().is_signed = true;
            let entry = DalPtr::new(entry);
            self.type_table
                .insert(entry.borrow().name.clone(), entry.clone());
            self.builtin_types.entry_isize = entry;
        }
        {
            let mut entry = TypeTableEntry::new(TypeTableEntryKind::Float);
            entry.type_ref = llvm::float_type();
            entry.name = "f32".to_string();
            entry.size_in_bits = 32;
            entry.align_in_bits = 32;
            let entry = DalPtr::new(entry);
            self.type_table
                .insert(entry.borrow().name.clone(), entry.clone());
            self.builtin_types.entry_f32 = entry;
        }
        {
            let mut entry = TypeTableEntry::new(TypeTableEntryKind::Void);
            entry.type_ref = llvm::void_type();
            entry.name = "void".to_string();
            let entry = DalPtr::new(entry);
            self.type_table
                .insert(entry.borrow().name.clone(), entry.clone());
            self.builtin_types.entry_void = entry;
        }
    }

    fn init(&mut self, source_path: String) {
        self.lib_search_paths.push(self.root_source_dir.to_string());
        self.lib_search_paths.push(DAL_STD_DIR.to_string());

        llvm::initialize_all_targets();
        llvm::initialize_all_target_mcs();
        llvm::initialize_all_asm_printers();
        llvm::initialize_all_asm_parsers();
        llvm::initialize_native_target();

        self.is_native_target = true;
        let native_triple = llvm::get_default_target_triple();

        self.module = llvm::module_create_with_name(&source_path);

        llvm::set_target(self.module, &native_triple);

        let mut target_ref: llvm::LLVMTargetRef = null_mut();
        let mut err_msg = String::new();
        if llvm::get_target_from_triple(&native_triple, &mut target_ref, &mut err_msg) {
            panic!("Failed to get target from triple: {}", err_msg);
        }

        let native_cpu = llvm::get_host_cpu_name();
        let native_features = llvm::get_host_cpu_features();
        let opt_level: llvm::LLVMCodegenOptLevel;
        if self.build_type == CodeGenBuildType::Debug {
            opt_level = llvm::LLVMCodegenOptLevel::LLVMCodeGenLevelNone;
        } else {
            opt_level = llvm::LLVMCodegenOptLevel::LLVMCodeGenLevelAggressive;
        }
        let reloc_mode: llvm::LLVMRelocMode;
        if self.is_static {
            reloc_mode = llvm::LLVMRelocMode::LLVMRelocStatic;
        } else {
            reloc_mode = llvm::LLVMRelocMode::LLVMRelocPIC;
        }
        self.target_machine = llvm::create_target_machine(
            target_ref,
            &native_triple,
            &native_cpu,
            &native_features,
            opt_level,
            reloc_mode,
            llvm::LLVMCodeModel::LLVMCodeModelDefault,
        );
        self.target_data_ref = llvm::create_target_data_layout(self.target_machine);
        let layout_str = llvm::copy_string_rep_of_target_data(self.target_data_ref);
        llvm::set_data_layout(self.module, &layout_str);

        self.pointer_size_bytes = llvm::pointer_size(self.target_data_ref).into();
        self.builder = llvm::create_builder();
        self.dbuilder = llvm::di_create_di_builder(self.module);

        self.define_primitive_types();

        let producer = format!("dal {}", DAL_VERSION);
        let is_optimized = self.build_type == CodeGenBuildType::Release;
        let flags = String::new();
        let runtime_version = 0;
        self.compile_unit = llvm::di_create_compile_unit(
            self.dbuilder,
            llvm::LLVMDWARFSourceLanguage::LLVMDWARFSourceLanguageC99,
            llvm::di_create_di_file(self.dbuilder, &source_path, &self.root_source_dir),
            &producer,
            is_optimized,
            &flags,
            runtime_version,
            "",
            llvm::LLVMDWARFEmissionKind::LLVMDWARFEmissionKindNone,
            0,
            !self.strip_debug_symbols,
            !self.strip_debug_symbols,
            "",
            "",
        );
    }

    fn add_code(
        &mut self,
        source_path: String,
        source_code: String,
    ) -> DalWeakPtr<ImportTableEntry> {
        let path = Path::new(&self.root_source_dir).join(&source_path);

        let dirname = path.parent().unwrap().to_str().unwrap().to_string();
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();

        if self.verbose {
            println!("\nOriginal source ({}):", source_path);
            println!("----------------------------------------");
            println!("{}\n", source_code);
            println!("\nTokens:");
            println!("----------------------------------------");
        }

        let mut tokenization = Tokenization::new();
        tokenize(&source_code, &mut tokenization);

        if let Some(msg) = tokenization.errors {
            let mut err = ErrMsg::new();
            err.line_start = tokenization.error_line;
            err.col_start = tokenization.error_col;
            err.msg = msg;
            err.path = source_path.clone();
            err.src = source_code.clone();
            err.line_offsets = tokenization.line_offsets.clone();
            err.print(self.err_color.clone());
        }

        if self.verbose {
            print_tokens(&source_code, &tokenization.tokens);
            println!("\nAST:");
            println!("----------------------------------------");
        }

        let import_entry = DalPtr::new(ImportTableEntry::new());
        {
            let mut entry = import_entry.borrow_mut();
            entry.src = source_code.clone();
            entry.line_offsets = tokenization.line_offsets.clone();
            entry.path = source_path.clone();
            entry.root = DalPtr::new(parse(
                &source_code,
                tokenization.tokens,
                import_entry.downgrade(),
            ));
            entry.di_file = llvm::di_create_di_file(self.dbuilder, &filename, &dirname);
        }
        if self.verbose {
            println!("{}", &import_entry.borrow().root.borrow());
        }
        {
            let it = self.import_table.clone();
            it.borrow_mut()
                .insert(source_path.clone(), import_entry.clone());
        }

        assert_eq!(import_entry.borrow().root.borrow().kind, NodeKind::Root);
        let mut err_found = false;
        for child in &import_entry.borrow().root.borrow().data.root().children {
            if err_found {
                break;
            }
            if child.borrow().kind == NodeKind::Import {
                let import_target_path = child.borrow().data.import().path.clone();
                let none_entry: bool;
                {
                    let it = self.import_table.borrow();
                    none_entry = it.get(&import_target_path).is_none();
                }
                if none_entry {
                    let mut full_path: String;
                    let import_code: String;
                    let mut found = false;

                    for search_path in self.lib_search_paths.clone() {
                        full_path = Path::new(&search_path)
                            .join(&import_target_path)
                            .with_extension("dal")
                            .to_str()
                            .unwrap()
                            .to_string();

                        match std::fs::read_to_string(&full_path) {
                            Ok(code) => {
                                import_code = code;
                            }
                            Err(e) => {
                                if e.kind() == std::io::ErrorKind::NotFound {
                                    continue;
                                } else {
                                    self.add_node_error(
                                        &child.borrow(),
                                        format!("unable to open '{}': {}", &full_path, e),
                                    );
                                    err_found = true;
                                    break;
                                }
                            }
                        }
                        found = true;
                        self.add_code(child.borrow().data.import().path.clone(), import_code);
                        break;
                    }
                    if err_found {
                        break;
                    }
                    if !found {
                        self.add_node_error(
                            &child.borrow(),
                            format!("unable to find import '{}'", &import_target_path),
                        );
                    }
                }
            } else if child.borrow().kind == NodeKind::FnDef {
                let proto = &child.borrow();
                let proto = &proto.data.fn_def().proto;
                assert_eq!(proto.kind, NodeKind::FnProto);
                let name = proto.data.fn_proto().name.clone();
                let is_public = proto.data.fn_proto().visib_mod == FnProtoVisibMod::Public;

                if name == "main" && is_public {
                    self.insert_bootstrap_code = true;
                }
            }
        }
        return import_entry.downgrade();
    }

    pub fn add_root_code(&mut self, source_path: String, source_code: String) {
        self.init(source_path.clone());

        self.root_import = self.add_code(source_path, source_code);

        if self.insert_bootstrap_code {
            let bootstrap_path = format!("{}/bootstrap.dal", DAL_STD_DIR);
            match std::fs::read_to_string(&bootstrap_path) {
                Err(kind) => {
                    panic!("unable to open '{}': {}", bootstrap_path, kind);
                }
                Ok(code) => {
                    self.add_code(bootstrap_path, code);
                }
            }
        }

        if self.verbose {
            println!("\nSemantic Analysis:");
            println!("----------------------------------------");
        }
        self.semantic_analyze();
    }
}
