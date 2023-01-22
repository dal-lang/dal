/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/ast.hh>

namespace dal::core {

//----------ast----------//
void ast::set_span(const span &ast_span) {
  this->m_span = ast_span;
}

span ast::ast_span() const {
  return this->m_span;
}

//----------string_ast----------//
ast_kind string_ast::kind() const {
  return this->m_kind;
}

void string_ast::set_value(const std::string &value) {
  this->m_value = value;
}

std::string string_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "string_ast: " + this->m_value + "\n";
}

//----------int_ast----------//
ast_kind int_ast::kind() const {
  return this->m_kind;
}

void int_ast::set_value(const std::string &value) {
  this->m_value = value;
}

std::string int_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "int_ast: " + this->m_value + "\n";
}

//----------bool_ast----------//
ast_kind bool_ast::kind() const {
  return this->m_kind;
}

void bool_ast::set_value(bool value) {
  this->m_value = value;
}

std::string bool_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "bool_ast: " + (this->m_value ? "true" : "false") + "\n";
}

//----------ident_ast----------//
ast_kind ident_ast::kind() const {
  return this->m_kind;
}

void ident_ast::set_name(const std::string &name) {
  this->m_name = name;
}

std::string ident_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "ident_ast: " + this->m_name + "\n";
}

//----------void_ast----------//
ast_kind void_ast::kind() const {
  return this->m_kind;
}

std::string void_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "void_ast\n";
}

//----------no_ret_ast----------//
ast_kind no_ret_ast::kind() const {
  return this->m_kind;
}

std::string no_ret_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "no_ret_ast\n";
}

//----------attr_ast----------//
ast_kind attr_ast::kind() const {
  return this->m_kind;
}

void attr_ast::set_name(const std::string &name) {
  this->m_name = name;
}

void attr_ast::set_arg(const std::string &arg) {
  this->m_arg = arg;
}

std::string attr_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "attr_ast:\n";
  str += std::string(indent + 2, ' ') + "name: " + this->m_name + "\n";
  str += std::string(indent + 2, ' ') + "arg: " + this->m_arg + "\n";
  return str;
}

//----------root_ast----------//
ast_kind root_ast::kind() const {
  return this->m_kind;
}

void root_ast::add_child(std::unique_ptr<ast> child) {
  this->m_children.push_back(std::move(child));
}

std::string root_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "root_ast:\n";
  for (const auto &child : this->m_children) {
    str += child->to_string(indent + 2);
  }
  return str;
}

//----------import_ast----------//
ast_kind import_ast::kind() const {
  return this->m_kind;
}

void import_ast::set_path(std::unique_ptr<string_ast> path) {
  this->m_path = std::move(path);
}

std::string import_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "import_ast:\n";
  str += this->m_path->to_string(indent + 2);
  return str;
}

//----------type_ast----------//
ast_kind type_ast::kind() const {
  return this->m_kind;
}

void type_ast::set_type_kind(const type_kind &kind) {
  this->m_type_kind = kind;
}

void type_ast::set_type_name(std::unique_ptr<ident_ast> name) {
  this->m_primitive = std::move(name);
}

void type_ast::set_type_child(std::unique_ptr<type_ast> child) {
  this->m_child = std::move(child);
}

void type_ast::set_type_size(std::unique_ptr<int_ast> size) {
  this->m_size = std::move(size);
}

void type_ast::set_mutability(bool is_mutable) {
  this->m_is_mut = is_mutable;
}

std::string type_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "type_ast:\n";
  str += std::string(indent + 2, ' ') + "type_kind: " + type_kind_to_string(this->m_type_kind) + "\n";
  if (this->m_primitive) {
    str += this->m_primitive->to_string(indent + 2);
  }
  if (this->m_child) {
    str += this->m_child->to_string(indent + 2);
  }
  if (this->m_size) {
    str += this->m_size->to_string(indent + 2);
  }
  str += std::string(indent + 2, ' ') + "is_mut: " + (this->m_is_mut ? "true" : "false") + "\n";
  return str;
}

//----------fn_param_ast----------//
ast_kind fn_param_ast::kind() const {
  return this->m_kind;
}

void fn_param_ast::set_name(std::unique_ptr<ident_ast> name) {
  this->m_name = std::move(name);
}

void fn_param_ast::set_type(std::unique_ptr<type_ast> type) {
  this->m_type = std::move(type);
}

std::string fn_param_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "fn_param_ast:\n";
  str += this->m_name->to_string(indent + 2);
  str += this->m_type->to_string(indent + 2);
  return str;
}

//----------fn_proto_ast----------//
ast_kind fn_proto_ast::kind() const {
  return this->m_kind;
}

void fn_proto_ast::set_name(std::unique_ptr<ident_ast> name) {
  this->m_name = std::move(name);
}

void fn_proto_ast::set_params(std::vector<std::unique_ptr<fn_param_ast>> params) {
  this->m_params = std::move(params);
}

void fn_proto_ast::set_return_type(std::unique_ptr<type_ast> return_type) {
  this->m_return_type = std::move(return_type);
}

void fn_proto_ast::set_public(bool is_public) {
  this->m_is_pub = is_public;
}

void fn_proto_ast::set_variadic(bool is_variadic) {
  this->m_is_variadic = is_variadic;
}

void fn_proto_ast::set_attrs(std::vector<std::unique_ptr<attr_ast>> attrs) {
  this->m_attrs = std::move(attrs);
}

std::string fn_proto_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "fn_proto_ast:\n";
  str += this->m_name->to_string(indent + 2);
  for (const auto &param : this->m_params) {
    str += param->to_string(indent + 2);
  }
  str += this->m_return_type->to_string(indent + 2);
  str += std::string(indent + 2, ' ') + "is_pub: " + (this->m_is_pub ? "true" : "false") + "\n";
  str += std::string(indent + 2, ' ') + "is_variadic: " + (this->m_is_variadic ? "true" : "false") + "\n";
  for (const auto &attr : this->m_attrs) {
    str += attr->to_string(indent + 2);
  }
  return str;
}

//----------fn_decl_ast----------//
ast_kind fn_decl_ast::kind() const {
  return this->m_kind;
}

void fn_decl_ast::set_proto(std::unique_ptr<fn_proto_ast> proto) {
  this->m_proto = std::move(proto);
}

std::string fn_decl_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "fn_decl_ast:\n";
  str += this->m_proto->to_string(indent + 2);
  return str;
}

//----------block_ast----------//
ast_kind block_ast::kind() const {
  return this->m_kind;
}

void block_ast::add_child(std::unique_ptr<ast> child) {
  this->m_children.push_back(std::move(child));
}

std::string block_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "block_ast:\n";
  for (const auto &child : this->m_children) {
    str += child->to_string(indent + 2);
  }
  return str;
}

//----------fn_def_ast----------//
ast_kind fn_def_ast::kind() const {
  return this->m_kind;
}

void fn_def_ast::set_proto(std::unique_ptr<fn_proto_ast> proto) {
  this->m_proto = std::move(proto);
}

void fn_def_ast::set_block(std::unique_ptr<block_ast> block) {
  this->m_body = std::move(block);
}

std::string fn_def_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "fn_def_ast:\n";
  str += this->m_proto->to_string(indent + 2);
  str += this->m_body->to_string(indent + 2);
  return str;
}

//----------extern_ast----------//
ast_kind extern_ast::kind() const {
  return this->m_kind;
}

void extern_ast::set_attrs(std::vector<std::unique_ptr<attr_ast>> attrs) {
  this->m_attrs = std::move(attrs);
}

void extern_ast::add_fn(std::unique_ptr<fn_decl_ast> fn) {
  this->m_fns.push_back(std::move(fn));
}

std::string extern_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "extern_ast:\n";
  for (const auto &attr : this->m_attrs) {
    str += attr->to_string(indent + 2);
  }
  for (const auto &fn : this->m_fns) {
    str += fn->to_string(indent + 2);
  }
  return str;
}

//----------var_decl_ast----------//
ast_kind var_decl_ast::kind() const {
  return this->m_kind;
}

void var_decl_ast::set_name(std::unique_ptr<ident_ast> name) {
  this->m_name = std::move(name);
}

void var_decl_ast::set_type(std::unique_ptr<type_ast> type) {
  this->m_type = std::move(type);
}

void var_decl_ast::set_value(std::unique_ptr<ast> value) {
  this->m_value = std::move(value);
}

void var_decl_ast::set_mutability(bool is_mutable) {
  this->m_is_mut = is_mutable;
}

std::string var_decl_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "var_decl_ast:\n";
  str += this->m_name->to_string(indent + 2);
  if (this->m_type) {
    str += this->m_type->to_string(indent + 2);
  }
  str += std::string(indent + 2, ' ') + "is_mut: " + (this->m_is_mut ? "true" : "false") + "\n";
  if (this->m_value) {
    str += this->m_value->to_string(indent + 2);
  }
  return str;
}

//----------return_ast----------//
ast_kind return_ast::kind() const {
  return this->m_kind;
}

void return_ast::set_value(std::unique_ptr<ast> value) {
  this->m_value = std::move(value);
}

std::string return_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "return_ast:\n";
  if (this->m_value) {
    str += this->m_value->to_string(indent + 2);
  }
  return str;
}

//----------call_ast----------//
ast_kind call_ast::kind() const {
  return this->m_kind;
}

void call_ast::set_ident(std::unique_ptr<ident_ast> ident) {
  this->m_name = std::move(ident);
}

void call_ast::set_args(std::vector<std::unique_ptr<ast>> args) {
  this->m_args = std::move(args);
}

std::string call_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "call_ast:\n";
  str += this->m_name->to_string(indent + 2);
  for (const auto &arg : this->m_args) {
    str += arg->to_string(indent + 2);
  }
  return str;
}

//----------if_ast----------//
ast_kind if_ast::kind() const {
  return this->m_kind;
}

void if_ast::set_cond(std::unique_ptr<ast> cond) {
  this->m_cond = std::move(cond);
}

void if_ast::set_then(std::unique_ptr<block_ast> then) {
  this->m_then = std::move(then);
}

void if_ast::set_else(std::unique_ptr<ast> else_) {
  this->m_else = std::move(else_);
}

std::string if_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "if_ast:\n";
  str += this->m_cond->to_string(indent + 2);
  str += this->m_then->to_string(indent + 2);
  if (this->m_else) {
    str += this->m_else->to_string(indent + 2);
  }
  return str;
}

//----------cast_ast----------//
ast_kind cast_ast::kind() const {
  return this->m_kind;
}

void cast_ast::set_type(std::unique_ptr<type_ast> type) {
  this->m_type = std::move(type);
}

void cast_ast::set_value(std::unique_ptr<ast> value) {
  this->m_value = std::move(value);
}

std::string cast_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "cast_ast:\n";
  str += this->m_type->to_string(indent + 2);
  str += this->m_value->to_string(indent + 2);
  return str;
}

//----------bin_op_ast----------//
ast_kind bin_op_ast::kind() const {
  return this->m_kind;
}

void bin_op_ast::set_op(bin_op_kind op) {
  this->m_op = op;
}

void bin_op_ast::set_lhs(std::unique_ptr<ast> lhs) {
  this->m_lhs = std::move(lhs);
}

void bin_op_ast::set_rhs(std::unique_ptr<ast> rhs) {
  this->m_rhs = std::move(rhs);
}

std::string bin_op_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "bin_op_ast:\n";
  str += std::string(indent + 2, ' ') + "op: " + bin_op_kind_to_string(this->m_op) + "\n";
  str += this->m_lhs->to_string(indent + 2);
  str += this->m_rhs->to_string(indent + 2);
  return str;
}

//----------un_op_ast----------//
ast_kind un_op_ast::kind() const {
  return this->m_kind;
}

void un_op_ast::set_op(un_op_kind op) {
  this->m_op = op;
}

void un_op_ast::set_value(std::unique_ptr<ast> value) {
  this->m_value = std::move(value);
}

std::string un_op_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "un_op_ast:\n";
  str += std::string(indent + 2, ' ') + "op: " + un_op_kind_to_string(this->m_op) + "\n";
  str += this->m_value->to_string(indent + 2);
  return str;
}

//----------array_index_ast----------//
ast_kind array_index_ast::kind() const {
  return this->m_kind;
}

void array_index_ast::set_array(std::unique_ptr<ident_ast> array) {
  this->m_array = std::move(array);
}

void array_index_ast::set_index(std::unique_ptr<ast> index) {
  this->m_index = std::move(index);
}

std::string array_index_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "array_index_ast:\n";
  str += this->m_array->to_string(indent + 2);
  str += this->m_index->to_string(indent + 2);
  return str;
}

// misc
static std::string type_kind_to_string(type_kind kind) {
  switch (kind) {
  case type_kind::primitive: return "primitive";
  case type_kind::array: return "array";
  case type_kind::pointer: return "pointer";
  }
}

static std::string ast_kind_to_string(ast_kind kind) {
  switch (kind) {
  case ast_kind::root_node: return "root_node";
  case ast_kind::import_node: return "import_node";
  case ast_kind::fn_proto_node: return "fn_proto_node";
  case ast_kind::fn_decl_node: return "fn_decl_node";
  case ast_kind::fn_def_node: return "fn_def_node";
  case ast_kind::fn_param_node: return "fn_param_node";
  case ast_kind::type_node: return "type_node";
  case ast_kind::block_node: return "block_node";
  case ast_kind::extern_node: return "extern_node";
  case ast_kind::attr_node: return "attr_node";
  case ast_kind::return_node: return "return_node";
  case ast_kind::var_decl_node: return "var_decl_node";
  case ast_kind::bin_op_node: return "bin_op_node";
  case ast_kind::un_op_node: return "un_op_node";
  case ast_kind::cast_node: return "cast_node";
  case ast_kind::no_ret_node: return "no_ret_node";
  case ast_kind::ident_node: return "ident_node";
  case ast_kind::call_node: return "call_node";
  case ast_kind::array_index_node: return "array_index_node";
  case ast_kind::void_node: return "void_node";
  case ast_kind::if_node: return "if_node";
  case ast_kind::string_node: return "string_node";
  case ast_kind::int_node: return "int_node";
  case ast_kind::bool_node: return "bool_node";
  }
}

static std::string bin_op_kind_to_string(bin_op_kind kind) {
  switch (kind) {
  case bin_op_kind::op_add: return "+";
  case bin_op_kind::op_sub: return "-";
  case bin_op_kind::op_mul: return "*";
  case bin_op_kind::op_div: return "/";
  case bin_op_kind::op_mod: return "%";
  case bin_op_kind::op_eq: return "==";
  case bin_op_kind::op_neq: return "!=";
  case bin_op_kind::op_lt: return "<";
  case bin_op_kind::op_gt: return ">";
  case bin_op_kind::op_lte: return "<=";
  case bin_op_kind::op_gte: return ">=";
  case bin_op_kind::op_and: return "&";
  case bin_op_kind::op_or: return "|";
  case bin_op_kind::op_xor: return "^";
  case bin_op_kind::op_shl: return "<<";
  case bin_op_kind::op_shr: return ">>";
  case bin_op_kind::op_assign: return "=";
  case bin_op_kind::op_log_and: return "&&";
  case bin_op_kind::op_log_or: return "||";
  }
}

static std::string un_op_kind_to_string(un_op_kind kind) {
  switch (kind) {
  case un_op_kind::op_neg: return "-";
  case un_op_kind::op_not: return "!";
  case un_op_kind::op_log_not: return "~";
  }
}

} // namespace dal::core