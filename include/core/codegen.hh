/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_CODEGEN_HH
#define DAL_CORE_CODEGEN_HH

#include <fmt/core.hh>
#include <string>
#include <vector>
#include "llvm.hh"
#include "table_entry.hh"

namespace dal::core {

enum class build_type {
  debug,
  release,
};

enum class out_type {
  exe,
  lib,
  obj,
};

class builtin_types {
 public:
  builtin_types() = default;
  ~builtin_types() = default;

  std::shared_ptr<type_table_entry> m_entry_invalid;
  std::shared_ptr<type_table_entry> m_entry_bool;
  std::shared_ptr<type_table_entry> m_entry_u8;
  std::shared_ptr<type_table_entry> m_entry_i32;
  std::shared_ptr<type_table_entry> m_entry_isize;
  std::shared_ptr<type_table_entry> m_entry_str_lit;
  std::shared_ptr<type_table_entry> m_entry_void;
  std::shared_ptr<type_table_entry> m_entry_no_return;
};

class codegen {
 public:
  codegen() = default;
  ~codegen() = default;

  void set_root_dir(const std::string& root_dir);
  void set_build_type(build_type type);
  void strip_debug_symbols();
  void link_static();
  void set_out_type(out_type type);
  void set_out_path(const std::string& path);
  void verbose();
  int gen(const std::string& path);

  std::shared_ptr<type_table_entry> get_ptr_to_type(
      const std::shared_ptr<type_table_entry>& child, bool is_const);

 private:
  std::string m_root_dir;
  std::string m_out_path;
  std::vector<std::string> m_lib_paths;
  build_type m_build_type = build_type::debug;
  out_type m_out_type = out_type::exe;
  builtin_types m_builtin_types;
  bool m_strip_debug_symbols = false;
  bool m_link_static = true;
  bool m_verbose = false;
  unsigned int m_ptr_size = 0;
  std::shared_ptr<llvm::Module> m_module;
  std::shared_ptr<llvm::TargetMachine> m_target_machine;
  std::shared_ptr<llvm::IRBuilder<>> m_builder;
  std::unordered_map<std::string, std::shared_ptr<type_table_entry>>
      m_type_table = {};
  std::unordered_map<std::string, std::shared_ptr<import_table>>
      m_import_table = {};
  std::shared_ptr<import_table> m_root_import;

  void init(const std::string& path);
  void set_primitive_types();
  std::shared_ptr<import_table> add_code(const std::string& path);
};

}  // namespace dal::core

#endif  //DAL_CORE_CODEGEN_HH
