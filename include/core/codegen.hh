/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_CODEGEN_HH
#define DAL_CORE_CODEGEN_HH

#include "table_entry.hh"
#include "error.hh"

namespace dal::core {

class block_ctx {
 public:
  block_ctx(std::shared_ptr<ast> node, std::shared_ptr<block_ctx> parent);
  ~block_ctx() = default;

  void set_node(const std::shared_ptr<ast> &node);
  void set_parent(const std::shared_ptr<block_ctx> &parent);
  void set_root(const std::shared_ptr<block_ctx> &root);
  void add_local_var(const std::string &name, const std::shared_ptr<local_var_table> &table);

  std::shared_ptr<ast> get_node();
  std::shared_ptr<block_ctx> get_parent();
  std::shared_ptr<block_ctx> get_root();
  std::shared_ptr<local_var_table> get_local_var(const std::string &name);
  [[nodiscard]] bool has_local_var(const std::string &name) const;

 private:
  std::shared_ptr<ast> m_node;
  std::shared_ptr<block_ctx> m_parent;
  std::shared_ptr<block_ctx> m_root;
  std::unordered_map<std::string, std::shared_ptr<local_var_table>> m_local_vars;
};

struct type_node {
  std::shared_ptr<type_table> table;
};

struct fn_proto_node {
  std::shared_ptr<fn_table> table;
};

struct fn_def_node {
  std::shared_ptr<type_table> implicit_ret_type;
  std::shared_ptr<block_ctx> block;
  bool skip;
  std::vector<std::shared_ptr<block_ctx>> blocks;
};

struct expr_node {
  std::shared_ptr<type_table> table;
  std::shared_ptr<block_ctx> block;
};

struct assign_node {
  std::shared_ptr<local_var_table> table;
};

struct block_node {
  std::shared_ptr<block_ctx> block;
};

class codegen_node {
 public:
  codegen_node() = default;
  ~codegen_node() = default;

  type_node m_type;
  fn_proto_node m_fn_proto;
  fn_def_node m_fn_def;
  assign_node m_assign;
  block_node m_block;
  expr_node m_expr;
};

enum class codegen_out_type {
  executable,
  library,
  object,
};

enum class codegen_build_type {
  debug,
  release,
};

class builtin_type {
 public:
  builtin_type() = default;
  ~builtin_type() = default;

  std::shared_ptr<type_table> m_invalid;
  std::shared_ptr<type_table> m_void;
  std::shared_ptr<type_table> m_bool;
  std::shared_ptr<type_table> m_no_return;
  std::shared_ptr<type_table> m_u8;
  std::shared_ptr<type_table> m_i32;
  std::shared_ptr<type_table> m_isize;
  std::shared_ptr<type_table> m_str_literal;
};

class codegen {
 public:
  codegen() = default;
  ~codegen() = default;

  // friend lists
  friend class analyze;

  void set_root_dir(const std::string &root_dir);
  void set_build_type(codegen_build_type build_type);
  void set_out_type(codegen_out_type out_type);
  void set_out_path(const std::string &out_path);
  void set_is_statically_linked(bool is_statically_linked);
  void set_is_verbose(bool is_verbose);
  void set_is_strip_symbols(bool is_strip_symbols);

  void generate(const std::string &file_path);

  std::shared_ptr<type_table> get_pointer_to_type(
      const std::shared_ptr<type_table> &type, bool is_const);

 private:
  std::string m_root_dir;
  std::string m_out_path;
  codegen_out_type m_out_type = codegen_out_type::executable;
  codegen_build_type m_build_type = codegen_build_type::debug;
  bool m_is_statically_linked = false;
  bool m_is_verbose = false;
  bool m_is_strip_symbols = false;

  builtin_type m_builtin_type;
  std::vector<std::string> m_dal_search_paths;

  // llvm related
  std::shared_ptr<llvm::LLVMContext> m_context;
  std::shared_ptr<llvm::Module> m_module;
  std::shared_ptr<llvm::IRBuilder<>> m_builder;
  std::shared_ptr<llvm::TargetMachine> m_target_machine;
  // analysis related
  unsigned int m_ptr_size = 0;
  std::unordered_map<std::string, std::shared_ptr<type_table>> m_type_table;
  std::shared_ptr<ast> m_root_ast;
  std::unordered_map<std::string, std::shared_ptr<import_table>> m_import_table;
  std::unordered_map<std::string, bool> m_link_table;
  std::unordered_map<std::string, std::shared_ptr<fn_table>> m_fn_table;
  std::vector<error> m_errors;

  // private methods
  void prepare(const std::string &file_path);
  void prepare_builtin_type();
  std::shared_ptr<ast> add_code(const std::string &file_path);
  void add_error(ast *node, const std::string &msg);
};

}  // namespace dal::core

#endif  //DAL_CORE_CODEGEN_HH
