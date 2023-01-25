/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/ast.hh>
#include <core/visitor.hh>

namespace dal::core {

static std::string type_kind_to_string(type_kind kind) {
  switch (kind) {
    case type_kind::primitive:
      return "primitive";
    case type_kind::array:
      return "array";
    case type_kind::pointer:
      return "pointer";
  }
  std::abort();
}

static std::string bin_op_kind_to_string(bin_op_kind kind) {
  switch (kind) {
    case bin_op_kind::op_add:
      return "+";
    case bin_op_kind::op_sub:
      return "-";
    case bin_op_kind::op_mul:
      return "*";
    case bin_op_kind::op_div:
      return "/";
    case bin_op_kind::op_mod:
      return "%";
    case bin_op_kind::op_eq:
      return "==";
    case bin_op_kind::op_neq:
      return "!=";
    case bin_op_kind::op_lt:
      return "<";
    case bin_op_kind::op_gt:
      return ">";
    case bin_op_kind::op_lte:
      return "<=";
    case bin_op_kind::op_gte:
      return ">=";
    case bin_op_kind::op_and:
      return "&";
    case bin_op_kind::op_or:
      return "|";
    case bin_op_kind::op_xor:
      return "^";
    case bin_op_kind::op_shl:
      return "<<";
    case bin_op_kind::op_shr:
      return ">>";
    case bin_op_kind::op_assign:
      return "=";
    case bin_op_kind::op_log_and:
      return "&&";
    case bin_op_kind::op_log_or:
      return "||";
  }
  std::abort();
}

static std::string un_op_kind_to_string(un_op_kind kind) {
  switch (kind) {
    case un_op_kind::op_neg:
      return "-";
    case un_op_kind::op_not:
      return "!";
    case un_op_kind::op_log_not:
      return "~";
  }
  std::abort();
}

//----------string_ast----------//
std::string string_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "string_ast: " + this->m_value + "\n";
}
void string_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind string_ast::kind() const {
  return this->m_kind;
}

//----------int_ast----------//
std::string int_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "int_ast: " + this->m_value + "\n";
}
void int_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind int_ast::kind() const {
  return this->m_kind;
}

//----------bool_ast----------//
std::string bool_ast::to_string(int indent) const {
  return std::string(indent, ' ') +
         "bool_ast: " + (this->m_value ? "true" : "false") + "\n";
}
void bool_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind bool_ast::kind() const {
  return this->m_kind;
}

//----------ident_ast----------//
std::string ident_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "ident_ast: " + this->m_name + "\n";
}
void ident_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind ident_ast::kind() const {
  return this->m_kind;
}

//----------void_ast----------//
std::string void_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "void_ast\n";
}
void void_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind void_ast::kind() const {
  return this->m_kind;
}

//----------no_ret_ast----------//
std::string no_ret_ast::to_string(int indent) const {
  return std::string(indent, ' ') + "no_ret_ast\n";
}
void no_ret_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind no_ret_ast::kind() const {
  return this->m_kind;
}

//----------attr_ast----------//
std::string attr_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "attr_ast:\n";
  str += std::string(indent + 2, ' ') + "name: " + this->m_name + "\n";
  str += std::string(indent + 2, ' ') + "arg: " + this->m_arg + "\n";
  return str;
}
void attr_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind attr_ast::kind() const {
  return this->m_kind;
}

//----------root_ast----------//
std::string root_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "root_ast:\n";
  for (const auto& child : this->m_children) {
    str += child->to_string(indent + 2);
  }
  return str;
}
void root_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind root_ast::kind() const {
  return this->m_kind;
}

//----------import_ast----------//
std::string import_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "import_ast:\n";
  str += this->m_path->to_string(indent + 2);
  return str;
}
void import_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind import_ast::kind() const {
  return this->m_kind;
}

//----------type_ast----------//
std::string type_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "type_ast:\n";
  str += std::string(indent + 2, ' ') +
         "type_kind: " + type_kind_to_string(this->m_type_kind) + "\n";
  if (this->m_primitive) {
    str += this->m_primitive->to_string(indent + 2);
  }
  if (this->m_child) {
    str += this->m_child->to_string(indent + 2);
  }
  if (this->m_size) {
    str += this->m_size->to_string(indent + 2);
  }
  str += std::string(indent + 2, ' ') +
         "is_mut: " + (this->m_is_mut ? "true" : "false") + "\n";
  return str;
}
void type_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind type_ast::kind() const {
  return this->m_kind;
}

//----------fn_param_ast----------//
std::string fn_param_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "fn_param_ast:\n";
  str += this->m_name->to_string(indent + 2);
  str += this->m_type->to_string(indent + 2);
  return str;
}
void fn_param_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind fn_param_ast::kind() const {
  return this->m_kind;
}

//----------fn_proto_ast----------//
std::string fn_proto_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "fn_proto_ast:\n";
  str += this->m_name->to_string(indent + 2);
  for (const auto& param : this->m_params) {
    str += param->to_string(indent + 2);
  }
  str += this->m_return_type->to_string(indent + 2);
  str += std::string(indent + 2, ' ') +
         "is_pub: " + (this->m_is_pub ? "true" : "false") + "\n";
  str += std::string(indent + 2, ' ') +
         "is_variadic: " + (this->m_is_variadic ? "true" : "false") + "\n";
  for (const auto& attr : this->m_attrs) {
    str += attr->to_string(indent + 2);
  }
  return str;
}
void fn_proto_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind fn_proto_ast::kind() const {
  return this->m_kind;
}

//----------fn_decl_ast----------//
std::string fn_decl_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "fn_decl_ast:\n";
  str += this->m_proto->to_string(indent + 2);
  return str;
}
void fn_decl_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind fn_decl_ast::kind() const {
  return this->m_kind;
}

//----------block_ast----------//
std::string block_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "block_ast:\n";
  for (const auto& child : this->m_children) {
    str += child->to_string(indent + 2);
  }
  return str;
}
void block_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind block_ast::kind() const {
  return this->m_kind;
}

//----------fn_def_ast----------//
std::string fn_def_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "fn_def_ast:\n";
  str += this->m_proto->to_string(indent + 2);
  str += this->m_body->to_string(indent + 2);
  return str;
}
void fn_def_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind fn_def_ast::kind() const {
  return this->m_kind;
}

//----------extern_ast----------//
std::string extern_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "extern_ast:\n";
  for (const auto& attr : this->m_attrs) {
    str += attr->to_string(indent + 2);
  }
  for (const auto& fn : this->m_fns) {
    str += fn->to_string(indent + 2);
  }
  return str;
}
void extern_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind extern_ast::kind() const {
  return this->m_kind;
}

//----------var_decl_ast----------//
std::string var_decl_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "var_decl_ast:\n";
  str += this->m_name->to_string(indent + 2);
  if (this->m_type) {
    str += this->m_type->to_string(indent + 2);
  }
  str += std::string(indent + 2, ' ') +
         "is_mut: " + (this->m_is_mut ? "true" : "false") + "\n";
  if (this->m_value) {
    str += this->m_value->to_string(indent + 2);
  }
  return str;
}
void var_decl_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind var_decl_ast::kind() const {
  return this->m_kind;
}

//----------return_ast----------//
std::string return_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "return_ast:\n";
  if (this->m_value) {
    str += this->m_value->to_string(indent + 2);
  }
  return str;
}
void return_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind return_ast::kind() const {
  return this->m_kind;
}

//----------call_ast----------//
std::string call_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "call_ast:\n";
  str += this->m_name->to_string(indent + 2);
  for (const auto& arg : this->m_args) {
    str += arg->to_string(indent + 2);
  }
  return str;
}
void call_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind call_ast::kind() const {
  return this->m_kind;
}

//----------if_ast----------//
std::string if_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "if_ast:\n";
  str += this->m_cond->to_string(indent + 2);
  str += this->m_then->to_string(indent + 2);
  if (this->m_else) {
    str += this->m_else->to_string(indent + 2);
  }
  return str;
}
void if_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind if_ast::kind() const {
  return this->m_kind;
}

//----------cast_ast----------//
std::string cast_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "cast_ast:\n";
  str += this->m_type->to_string(indent + 2);
  str += this->m_value->to_string(indent + 2);
  return str;
}
void cast_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind cast_ast::kind() const {
  return this->m_kind;
}

//----------bin_op_ast----------//
std::string bin_op_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "bin_op_ast:\n";
  str += std::string(indent + 2, ' ') +
         "op: " + bin_op_kind_to_string(this->m_op) + "\n";
  str += this->m_lhs->to_string(indent + 2);
  str += this->m_rhs->to_string(indent + 2);
  return str;
}
void bin_op_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind bin_op_ast::kind() const {
  return this->m_kind;
}

//----------un_op_ast----------//
std::string un_op_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "un_op_ast:\n";
  str += std::string(indent + 2, ' ') +
         "op: " + un_op_kind_to_string(this->m_op) + "\n";
  str += this->m_value->to_string(indent + 2);
  return str;
}
void un_op_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind un_op_ast::kind() const {
  return this->m_kind;
}

//----------array_index_ast----------//
std::string array_index_ast::to_string(int indent) const {
  auto str = std::string(indent, ' ') + "array_index_ast:\n";
  str += this->m_array->to_string(indent + 2);
  str += this->m_index->to_string(indent + 2);
  return str;
}
void array_index_ast::accept(ast_visitor::visitor& v) {
  v.visit(this);
}
ast_kind array_index_ast::kind() const {
  return this->m_kind;
}

}  // namespace dal::core