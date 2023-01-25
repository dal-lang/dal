/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_PARSER_HH
#define DAL_CORE_PARSER_HH

#include "ast.hh"
#include "error.hh"
#include "table_entry.hh"
#include "token.hh"

namespace dal::core {

class parser {
 public:
  parser(const std::string& source, const std::vector<token>& tokens,
         std::shared_ptr<import_table> owner);
  ~parser() = default;

  ast* parse();

 private:
  std::string m_source;
  std::shared_ptr<import_table> m_owner;
  std::vector<token> m_tokens;
  std::vector<std::shared_ptr<attr_ast>> m_attrs;
  unsigned long m_index = 0;

  fn_def_ast* parse_fn_def(bool is_required);
  extern_ast* parse_extern(bool is_required);
  import_ast* parse_import(bool is_required);
  fn_proto_ast* parse_fn_proto(bool is_required);
  block_ast* parse_block(bool is_required);
  ident_ast* parse_ident(bool is_required);
  fn_param_ast* parse_fn_param();
  string_ast* parse_string();
  int_ast* parse_int();
  bool_ast* parse_bool();
  type_ast* parse_type();
  fn_decl_ast* parse_fn_decl();
  var_decl_ast* parse_var_decl(bool is_required);
  if_ast* parse_if(bool is_required);
  return_ast* parse_return(bool is_required);
  type_ast* create_prim_type(const span& type_span,
                             const std::string& type_name);
  void_ast* create_void_ast(const span& void_span);
  no_ret_ast* create_no_ret_ast(const span& no_ret_span);
  ast* parse_if_or_block(bool is_required);
  ast* parse_return_or_assign(bool is_required);
  ast* parse_expr(bool is_required);
  ast* parse_else(bool is_required);
  ast* parse_assign(bool is_required);
  ast* parse_log_or(bool is_required);
  ast* parse_log_and(bool is_required);
  ast* parse_comparison(bool is_required);
  ast* parse_bit_or(bool is_required);
  ast* parse_bit_xor(bool is_required);
  ast* parse_bit_and(bool is_required);
  ast* parse_bit_shift(bool is_required);
  ast* parse_add(bool is_required);
  ast* parse_mul(bool is_required);
  ast* parse_cast(bool is_required);
  ast* parse_unary(bool is_required);
  ast* parse_postfix(bool is_required);
  ast* parse_primary(bool is_required);
  ast* parse_group(bool is_required);
  std::vector<std::shared_ptr<fn_param_ast>> parse_fn_params(bool* is_variadic);
  std::vector<std::shared_ptr<ast>> parse_call_args();
  [[nodiscard]] std::string tok_value(const token& tok) const;
  [[noreturn]] void error(const token& tok, const std::string& msg);
  [[noreturn]] void error(const span& span, const std::string& msg);
  void expect(const token& tok, token_kind kind);
  void parse_attrs();
  void parse_attr();
};

}  // namespace dal::core

#endif  //DAL_CORE_PARSER_HH
