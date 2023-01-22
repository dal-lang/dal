/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_PARSER_HH
#define DAL_CORE_PARSER_HH

#include "token.hh"
#include "ast.hh"
#include "error.hh"

namespace dal::core {

class parser {
public:
  parser(const std::string &source, const std::vector<token> &tokens);
  ~parser() = default;

  std::unique_ptr<ast> parse();
private:
  std::string m_source;
  std::vector<token> m_tokens;
  std::vector<std::unique_ptr<attr_ast>> m_attrs;
  int m_index = 0;

  void parse_attrs();
  void parse_attr();
  std::unique_ptr<fn_def_ast> parse_fn_def(bool is_required);
  std::unique_ptr<extern_ast> parse_extern(bool is_required);
  std::unique_ptr<import_ast> parse_import(bool is_required);
  std::unique_ptr<fn_proto_ast> parse_fn_proto(bool is_required);
  std::unique_ptr<block_ast> parse_block(bool is_required);
  std::unique_ptr<ident_ast> parse_ident(bool is_required);
  std::vector<std::unique_ptr<fn_param_ast>> parse_fn_params(bool *is_variadic);
  std::unique_ptr<fn_param_ast> parse_fn_param();
  std::unique_ptr<string_ast> parse_string();
  std::unique_ptr<int_ast> parse_int();
  std::unique_ptr<bool_ast> parse_bool();
  std::unique_ptr<type_ast> parse_type();
  std::unique_ptr<fn_decl_ast> parse_fn_decl();
  std::unique_ptr<var_decl_ast> parse_var_decl(bool is_required);
  std::unique_ptr<ast> parse_if_or_block(bool is_required);
  std::unique_ptr<if_ast> parse_if(bool is_required);
  std::unique_ptr<ast> parse_return_or_assign(bool is_required);
  std::unique_ptr<ast> parse_expr(bool is_required);
  std::unique_ptr<ast> parse_else(bool is_required);
  std::unique_ptr<return_ast> parse_return(bool is_required);
  std::unique_ptr<ast> parse_assign(bool is_required);
  std::unique_ptr<ast> parse_log_or(bool is_required);
  std::unique_ptr<ast> parse_log_and(bool is_required);
  std::unique_ptr<ast> parse_comparison(bool is_required);
  std::unique_ptr<ast> parse_bit_or(bool is_required);
  std::unique_ptr<ast> parse_bit_xor(bool is_required);
  std::unique_ptr<ast> parse_bit_and(bool is_required);
  std::unique_ptr<ast> parse_bit_shift(bool is_required);
  std::unique_ptr<ast> parse_add(bool is_required);
  std::unique_ptr<ast> parse_mul(bool is_required);
  std::unique_ptr<ast> parse_cast(bool is_required);
  std::unique_ptr<ast> parse_unary(bool is_required);
  std::unique_ptr<ast> parse_postfix(bool is_required);
  std::unique_ptr<ast> parse_primary(bool is_required);
  std::vector<std::unique_ptr<ast>> parse_call_args();
  std::unique_ptr<ast> parse_group(bool is_required);

  [[nodiscard]] std::string tok_value(const token &tok) const;
  static std::unique_ptr<type_ast> create_prim_type(const span &type_span, const std::string &type_name);
  static std::unique_ptr<void_ast> create_void_ast(const span &void_span);
  static std::unique_ptr<no_ret_ast> create_no_ret_ast(const span &no_ret_span);

  [[noreturn]]
  void error(const token &tok, const std::string &msg);
  [[noreturn]]
  void error(const span &span, const std::string &msg);
  void expect(const token &tok, token_kind kind);
};

} // namespace dal::core

#endif //DAL_CORE_PARSER_HH
