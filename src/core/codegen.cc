/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <config.h>
#include <core/codegen.hh>
#include <core/os.hh>
#include <core/parser.hh>
#include <core/tokenizer.hh>

namespace dal::core {

void codegen::set_root_dir(const std::string& root_dir) {
  this->m_root_dir = root_dir;
}

void codegen::set_build_type(build_type type) {
  this->m_build_type = type;
}

void codegen::strip_debug_symbols() {
  this->m_strip_debug_symbols = true;
}

void codegen::link_static() {
  this->m_link_static = true;
}

void codegen::set_out_type(out_type type) {
  this->m_out_type = type;
}

void codegen::set_out_path(const std::string& path) {
  this->m_out_path = path;
}

void codegen::verbose() {
  this->m_verbose = true;
}

int codegen::gen(const std::string& path) {
  this->init(path);

  this->m_root_import = this->add_code(path);
  return 0;
}

void codegen::init(const std::string& path) {
  this->m_lib_paths.push_back(this->m_root_dir);
  this->m_lib_paths.emplace_back(DAL_STD_DIR);

  initialize_llvm();
  std::string triple = get_host_triple();
  this->m_module = std::move(new_module(path));
  this->m_module->setTargetTriple(triple);
  std::string error;
  auto target = llvm::TargetRegistry::lookupTarget(triple, error);
  if (!target) {
    fmt::panic("failed to lookup target: {}\n", error);
  }

  auto cpu = llvm::sys::getHostCPUName().str();
  auto features = get_host_cpu_features();
  llvm::CodeGenOpt::Level opt_level = (this->m_build_type == build_type::debug)
                                          ? llvm::CodeGenOpt::None
                                          : llvm::CodeGenOpt::Aggressive;
  llvm::Reloc::Model reloc_model =
      (this->m_link_static) ? llvm::Reloc::Static : llvm::Reloc::PIC_;
  this->m_target_machine = std::move(create_target_machine(
      *target, triple, cpu, features, reloc_model, opt_level));
  auto data_layout = this->m_target_machine->createDataLayout();
  this->m_module->setDataLayout(data_layout);
  this->m_ptr_size = data_layout.getPointerSize();
  this->m_builder = std::move(new_ir_builder(this->m_module->getContext()));

  this->set_primitive_types();
}

void codegen::set_primitive_types() {
  {
    // invalid type
    auto entry =
        std::make_shared<type_table_entry>(type_table_kind::type_invalid);
    entry->set_name("(invalid)");
    this->m_builtin_types.m_entry_invalid = entry;
  }
  {
    // bool type
    auto entry = std::make_shared<type_table_entry>(type_table_kind::type_bool);
    entry->set_name("bool");
    auto ty_ref = llvm::Type::getInt1Ty(this->m_module->getContext());
    entry->set_size(1);
    entry->set_align(8);
    entry->set_llvm_type(ty_ref);
    this->m_type_table.insert({entry->get_name(), entry});
    this->m_builtin_types.m_entry_bool = entry;
  }
  {
    // u8
    auto entry = std::make_shared<type_table_entry>(type_table_kind::type_int);
    entry->set_name("u8");
    auto ty_ref = llvm::Type::getInt8Ty(this->m_module->getContext());
    entry->set_size(1);
    entry->set_align(8);
    entry->set_llvm_type(ty_ref);
    this->m_type_table.insert({entry->get_name(), entry});
    this->m_builtin_types.m_entry_u8 = entry;
  }
  this->m_builtin_types.m_entry_str_lit =
      this->get_ptr_to_type(this->m_builtin_types.m_entry_u8, true);
  {
    // i32
    auto entry = std::make_shared<type_table_entry>(type_table_kind::type_int);
    entry->set_name("i32");
    auto ty_ref = llvm::Type::getInt32Ty(this->m_module->getContext());
    entry->set_size(32);
    entry->set_align(32);
    entry->set_llvm_type(ty_ref);
    entry->set_signed(true);
    this->m_type_table.insert({entry->get_name(), entry});
    this->m_builtin_types.m_entry_i32 = entry;
  }
  {
    // isize
    auto entry = std::make_shared<type_table_entry>(type_table_kind::type_int);
    entry->set_name("isize");
    auto ty_ref = llvm::Type::getIntNTy(this->m_module->getContext(),
                                        this->m_ptr_size * 8);
    entry->set_size(this->m_ptr_size * 8);
    entry->set_align(this->m_ptr_size * 8);
    entry->set_llvm_type(ty_ref);
    entry->set_signed(true);
    this->m_type_table.insert({entry->get_name(), entry});
    this->m_builtin_types.m_entry_isize = entry;
  }
  {
    // void
    auto entry = std::make_shared<type_table_entry>(type_table_kind::type_void);
    entry->set_name("void");
    auto ty_ref = llvm::Type::getVoidTy(this->m_module->getContext());
    entry->set_llvm_type(ty_ref);
    this->m_type_table.insert({entry->get_name(), entry});
    this->m_builtin_types.m_entry_void = entry;
  }
  {
    // ! (no return)
    auto entry =
        std::make_shared<type_table_entry>(type_table_kind::type_no_return);
    entry->set_name("!");
    auto ty_ref = llvm::Type::getVoidTy(this->m_module->getContext());
    entry->set_llvm_type(ty_ref);
    this->m_type_table.insert({entry->get_name(), entry});
    this->m_builtin_types.m_entry_no_return = entry;
  }
}

std::shared_ptr<type_table_entry> codegen::get_ptr_to_type(
    const std::shared_ptr<type_table_entry>& child, bool is_const) {
  auto parent_ptr =
      is_const ? child->get_const_parent_ptr() : child->get_mut_parent_ptr();
  if (parent_ptr) {
    return parent_ptr;
  } else {
    type_table_entry entry(type_table_kind::type_ptr);
    auto ty_ref = llvm::PointerType::get(child->get_llvm_type(), 0);
    entry.set_llvm_type(ty_ref);
    entry.set_size(this->m_ptr_size * 8);
    entry.set_align(this->m_ptr_size * 8);
    std::string name =
        fmt::format("*{} {}", is_const ? "const" : "mut", child->get_name());
    entry.set_name(name);
    auto entry_ptr = std::make_shared<type_table_entry>(entry);
    if (is_const) {
      child->set_const_parent_ptr(entry_ptr);
    } else {
      child->set_mut_parent_ptr(entry_ptr);
    }
    this->m_type_table.insert({name, entry_ptr});
    return entry_ptr;
  }
}

std::shared_ptr<import_table> codegen::add_code(const std::string& path) {
  if (this->m_verbose) {
    fmt::println("\n[+] adding code: {}", path);
    fmt::println("----------------------------------------");
    fmt::println("\n~~~~~~tokens:\n");
  }

  std::error_code ec;
  auto file_contents = os::read_file(path, ec);
  if (ec) {
    fmt::panic("{}: failed to read file: {}\n", fmt::red_bold("error"),
               ec.message());
  }

  tokenizer tk;
  auto tokens = tk.tokenize(file_contents, path);
  if (tk.has_error())
    tk.print_error();

  if (this->m_verbose) {
    for (auto& m_token : tokens) {
      fmt::println("  {}", m_token.t_str(file_contents));
    }
    fmt::println("\n~~~~~~ast:\n");
  }

  auto import_entry = std::make_shared<import_table>();
  import_entry->set_path(path);
  import_entry->set_source(file_contents);
  parser p(file_contents, tokens, import_entry);
  auto root = p.parse();
  root->set_owner(import_entry);
  import_entry->set_root(std::move(root));
  if (this->m_verbose) {
    auto str = import_entry->get_root()->to_string(0);
    fmt::println("{}", str);
  }

  this->m_import_table.insert({path, import_entry});

  // TODO: perform import resolution

  return import_entry;
}

}  // namespace dal::core