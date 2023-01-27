/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_TYPE_TABLE_HH
#define DAL_CORE_TYPE_TABLE_HH

#include "ast.hh"
#include "llvm.hh"

namespace dal::core {

enum class type_table_kind {
  type_invalid,
  type_void,
  type_bool,
  type_no_return,
  type_int,
  type_ptr,
  type_array,
};

class type_table {
 public:
  explicit type_table(type_table_kind kind);
  ~type_table() = default;

  void set_name(const std::string &name);
  void set_type(llvm::Type *type);
  void set_size(unsigned int size);
  void set_align(unsigned int align);
  void set_is_signed(bool is_signed);
  void set_pointee(std::shared_ptr<type_table> pointee);
  void set_pointee_is_const(bool pointee_is_const);
  void set_const_pointee(std::shared_ptr<type_table> const_pointee);
  void set_mut_pointee(std::shared_ptr<type_table> mut_pointee);
  void set_array(std::shared_ptr<type_table> array);
  void add_array_by_size(int size, std::shared_ptr<type_table> array);

  [[nodiscard]] type_table_kind get_kind() const;
  [[nodiscard]] std::string get_name() const;
  llvm::Type *get_type();
  [[nodiscard]] unsigned int get_size() const;
  [[nodiscard]] unsigned int get_align() const;
  [[nodiscard]] bool get_is_signed() const;
  std::shared_ptr<type_table> get_pointee();
  [[nodiscard]] bool get_pointee_is_const() const;
  std::shared_ptr<type_table> get_const_pointee();
  std::shared_ptr<type_table> get_mut_pointee();
  std::shared_ptr<type_table> get_array();
  std::shared_ptr<type_table> get_array_by_size(int size);

 private:
  type_table_kind m_kind;
  std::string m_name;
  llvm::Type *m_type = nullptr;
  unsigned int m_size = 0;
  unsigned int m_align = 0;
  // integer type
  bool m_is_signed = false;
  // pointer type
  std::shared_ptr<type_table> m_pointee;
  bool m_pointee_is_const = false;
  std::shared_ptr<type_table> m_const_pointee;
  std::shared_ptr<type_table> m_mut_pointee;
  // array type
  std::shared_ptr<type_table> m_array;
  std::unordered_map<int, std::shared_ptr<type_table>> m_array_by_size;
};

class fn_table;

class import_table {
 public:
  import_table() = default;
  ~import_table() = default;

  void set_source(const std::string &source);
  void set_path(const std::string &path);
  void set_root(std::shared_ptr<ast> root);
  void add_fn_table(const std::string &name,
                    std::shared_ptr<fn_table> fn_table);

  std::string get_source() const;
  std::string get_path() const;
  std::shared_ptr<ast> get_root() const;
  std::shared_ptr<fn_table> get_fn_table(const std::string &name) const;

 private:
  std::string m_source;
  std::string m_path;
  std::shared_ptr<ast> m_root;
  std::unordered_map<std::string, std::shared_ptr<fn_table>> m_fn_table;
};

enum class fn_attr_kind {
  none,
  always_inline,
};

class fn_table {
 public:
  fn_table() = default;
  ~fn_table() = default;

  void set_proto(std::shared_ptr<fn_proto_ast> proto);
  void set_def(std::shared_ptr<ast> def);
  void set_import(std::shared_ptr<import_table> import);
  void add_attr(fn_attr_kind attr);
  void set_calling_conv(llvm::CallingConv::ID calling_conv);
  void set_is_external(bool is_external);

  std::shared_ptr<fn_proto_ast> get_proto();
  std::shared_ptr<ast> get_def();
  std::shared_ptr<import_table> get_import();
  [[nodiscard]] std::vector<fn_attr_kind> get_attrs() const;
  [[nodiscard]] llvm::CallingConv::ID get_calling_conv() const;
  [[nodiscard]] bool get_is_external() const;

 private:
  std::shared_ptr<fn_proto_ast> m_proto;
  std::shared_ptr<ast> m_def;
  std::shared_ptr<import_table> m_import;
  std::vector<fn_attr_kind> m_attrs;
  llvm::CallingConv::ID m_calling_conv = llvm::CallingConv::Fast;
  bool m_is_external = false;
};

class local_var_table {
 public:
  local_var_table() = default;
  ~local_var_table() = default;

  void set_name(const std::string &name);
  void set_type(const std::shared_ptr<type_table> &type);
  void set_value(llvm::Value *value);
  void set_is_const(bool is_const);
  void set_is_ptr(bool is_ptr);
  void set_decl_node(const std::shared_ptr<ast> &decl_node);
  void set_arg_index(int arg_index);

  [[nodiscard]] std::string get_name() const;
  std::shared_ptr<type_table> get_type();
  llvm::Value *get_value();
  [[nodiscard]] bool get_is_const() const;
  [[nodiscard]] bool get_is_ptr() const;
  std::shared_ptr<ast> get_decl_node();
  [[nodiscard]] int get_arg_index() const;

 private:
  std::string m_name;
  std::shared_ptr<type_table> m_type;
  llvm::Value *m_value = nullptr;
  bool m_is_const = false;
  bool m_is_ptr = false;
  std::shared_ptr<ast> m_decl_node;
  int m_arg_index = -1;
};

}  // namespace dal::core

#endif  //DAL_CORE_TYPE_TABLE_HH
