/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_AST_HH
#define DAL_CORE_AST_HH

#include <memory>
#include <string>
#include <vector>
#include "span.hh"

namespace dal::core {

class import_table;
class ast;

namespace ast_visitor {
class visitor;
}

enum class ast_kind {
  root_node,
  import_node,
  fn_proto_node,
  fn_decl_node,
  fn_def_node,
  fn_param_node,
  type_node,
  block_node,
  extern_node,
  attr_node,
  return_node,
  var_decl_node,
  bin_op_node,
  un_op_node,
  cast_node,
  no_ret_node,
  ident_node,
  call_node,
  array_index_node,
  void_node,
  if_node,
  string_node,
  int_node,
  bool_node,
};

class ast {
 public:
  ast() = default;
  virtual ~ast() = default;

  virtual void accept(ast_visitor::visitor& v) = 0;
  [[nodiscard]] virtual std::string to_string(int indent) const = 0;
  [[nodiscard]] virtual ast_kind kind() const = 0;

  span m_span;
  std::weak_ptr<import_table> m_owner;
};

class string_ast : public ast {
 public:
  string_ast() = default;
  ~string_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::string_node;
  std::string m_value;
};

class int_ast : public ast {
 public:
  int_ast() = default;
  ~int_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::int_node;
  std::string m_value;
};

class bool_ast : public ast {
 public:
  bool_ast() = default;
  ~bool_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::bool_node;
  bool m_value = false;
};

class ident_ast : public ast {
 public:
  ident_ast() = default;
  ~ident_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::ident_node;
  std::string m_name;
};

class void_ast : public ast {
 public:
  void_ast() = default;
  ~void_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::void_node;
};

class no_ret_ast : public ast {
 public:
  no_ret_ast() = default;
  ~no_ret_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::no_ret_node;
};

class attr_ast : public ast {
 public:
  attr_ast() = default;
  ~attr_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::attr_node;
  std::string m_name;
  std::string m_arg;
};

class root_ast : public ast {
 public:
  root_ast() = default;
  ~root_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::root_node;
  std::vector<std::shared_ptr<ast>> m_children;
};

class import_ast : public ast {
 public:
  import_ast() = default;
  ~import_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::import_node;
  std::shared_ptr<string_ast> m_path;
};

enum class type_kind {
  primitive,
  array,
  pointer,
};

class type_ast : public ast {
 public:
  type_ast() = default;
  ~type_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::type_node;
  type_kind m_type_kind = type_kind::primitive;
  // primitive stuff
  std::shared_ptr<ident_ast> m_primitive;
  // array and pointer stuff
  std::shared_ptr<type_ast> m_child;
  std::shared_ptr<int_ast> m_size;  // only for array
  bool m_is_mut = false;            // only for pointer
};

class fn_param_ast : public ast {
 public:
  fn_param_ast() = default;
  ~fn_param_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::fn_param_node;
  std::shared_ptr<ident_ast> m_name;
  std::shared_ptr<type_ast> m_type;
};

class fn_proto_ast : public ast {
 public:
  fn_proto_ast() = default;
  ~fn_proto_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::fn_proto_node;
  std::shared_ptr<ident_ast> m_name;
  std::shared_ptr<type_ast> m_return_type;
  std::vector<std::shared_ptr<fn_param_ast>> m_params;
  std::vector<std::shared_ptr<attr_ast>> m_attrs;
  bool m_is_pub = false;
  bool m_is_variadic = false;
};

class fn_decl_ast : public ast {
 public:
  fn_decl_ast() = default;
  ~fn_decl_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::fn_decl_node;
  std::shared_ptr<fn_proto_ast> m_proto;
};

class block_ast : public ast {
 public:
  block_ast() = default;
  ~block_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::block_node;
  std::vector<std::shared_ptr<ast>> m_children;
};

class fn_def_ast : public ast {
 public:
  fn_def_ast() = default;
  ~fn_def_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::fn_def_node;
  std::shared_ptr<fn_proto_ast> m_proto;
  std::shared_ptr<block_ast> m_body;
};

class extern_ast : public ast {
 public:
  extern_ast() = default;
  ~extern_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::extern_node;
  std::vector<std::shared_ptr<attr_ast>> m_attrs;
  std::vector<std::shared_ptr<fn_decl_ast>> m_fns;
};

class var_decl_ast : public ast {
 public:
  var_decl_ast() = default;
  ~var_decl_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::var_decl_node;
  std::shared_ptr<ident_ast> m_name;
  // one of these can't be null
  std::shared_ptr<type_ast> m_type;
  std::shared_ptr<ast> m_value;
  bool m_is_mut = false;
};

class return_ast : public ast {
 public:
  return_ast() = default;
  ~return_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::return_node;
  std::shared_ptr<ast> m_value;
};

class call_ast : public ast {
 public:
  call_ast() = default;
  ~call_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::call_node;
  std::shared_ptr<ident_ast> m_name;
  std::vector<std::shared_ptr<ast>> m_args;
};

class if_ast : public ast {
 public:
  if_ast() = default;
  ~if_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::if_node;
  std::shared_ptr<ast> m_cond;
  std::shared_ptr<block_ast> m_then;
  std::shared_ptr<ast> m_else;
};

class cast_ast : public ast {
 public:
  cast_ast() = default;
  ~cast_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::cast_node;
  std::shared_ptr<type_ast> m_type;
  std::shared_ptr<ast> m_value;
};

enum class bin_op_kind {
  op_add,
  op_sub,
  op_mul,
  op_div,
  op_mod,
  op_eq,
  op_neq,
  op_lt,
  op_gt,
  op_lte,
  op_gte,
  op_and,
  op_or,
  op_xor,
  op_shl,
  op_shr,
  op_assign,
  op_log_and,
  op_log_or,
};

class bin_op_ast : public ast {
 public:
  bin_op_ast() = default;
  ~bin_op_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::bin_op_node;
  bin_op_kind m_op = bin_op_kind::op_add;
  std::shared_ptr<ast> m_lhs;
  std::shared_ptr<ast> m_rhs;
};

enum class un_op_kind {
  op_neg,
  op_not,
  op_log_not,
};

class un_op_ast : public ast {
 public:
  un_op_ast() = default;
  ~un_op_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::un_op_node;
  un_op_kind m_op = un_op_kind::op_neg;
  std::shared_ptr<ast> m_value;
};

class array_index_ast : public ast {
 public:
  array_index_ast() = default;
  ~array_index_ast() override = default;

  void accept(ast_visitor::visitor& v) override;
  [[nodiscard]] std::string to_string(int indent) const override;
  [[nodiscard]] ast_kind kind() const override;

  ast_kind m_kind = ast_kind::array_index_node;
  std::shared_ptr<ident_ast> m_array;
  std::shared_ptr<ast> m_index;
};

}  // namespace dal::core

#endif  //DAL_CORE_AST_HH
