/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/token.hh>

namespace dal::core {

token::token(token_kind t_kind, span t_span) {
  this->m_kind = t_kind;
  this->m_span = t_span;
}

token_kind token::t_kind() const {
  return this->m_kind;
}

span token::t_span() const {
  return this->m_span;
}
std::string token::t_kind_str() const {
  switch (this->t_kind()) {
  case token_kind::kw_let: return "kw_let";
  case token_kind::kw_const: return "kw_const";
  case token_kind::kw_mut: return "kw_mut";
  case token_kind::kw_if: return "kw_if";
  case token_kind::kw_else: return "kw_else";
  case token_kind::kw_fn: return "kw_fn";
  case token_kind::kw_return: return "kw_return";
  case token_kind::kw_pub: return "kw_pub";
  case token_kind::kw_true: return "kw_true";
  case token_kind::kw_false: return "kw_false";
  case token_kind::kw_extern: return "kw_extern";
  case token_kind::kw_import: return "kw_import";
  case token_kind::kw_as: return "kw_as";
  case token_kind::op_add: return "op_add";
  case token_kind::op_sub: return "op_sub";
  case token_kind::op_mul: return "op_mul";
  case token_kind::op_div: return "op_div";
  case token_kind::op_mod: return "op_mod";
  case token_kind::op_eq: return "op_eq";
  case token_kind::op_neq: return "op_neq";
  case token_kind::op_lt: return "op_lt";
  case token_kind::op_gt: return "op_gt";
  case token_kind::op_lte: return "op_lte";
  case token_kind::op_gte: return "op_gte";
  case token_kind::op_and: return "op_and";
  case token_kind::op_or: return "op_or";
  case token_kind::op_not: return "op_not";
  case token_kind::op_xor: return "op_xor";
  case token_kind::op_shl: return "op_shl";
  case token_kind::op_shr: return "op_shr";
  case token_kind::op_assign: return "op_assign";
  case token_kind::op_log_and: return "op_log_and";
  case token_kind::op_log_or: return "op_log_or";
  case token_kind::op_log_not: return "op_log_not";
  case token_kind::punct_lparen: return "punct_lparen";
  case token_kind::punct_rparen: return "punct_rparen";
  case token_kind::punct_lbrace: return "punct_lbrace";
  case token_kind::punct_rbrace: return "punct_rbrace";
  case token_kind::punct_lbracket: return "punct_lbracket";
  case token_kind::punct_rbracket: return "punct_rbracket";
  case token_kind::punct_comma: return "punct_comma";
  case token_kind::punct_colon: return "punct_colon";
  case token_kind::punct_semicolon: return "punct_semicolon";
  case token_kind::punct_dot: return "punct_dot";
  case token_kind::punct_arrow: return "punct_arrow";
  case token_kind::punct_ellipsis: return "punct_ellipsis";
  case token_kind::punct_at: return "punct_at";
  case token_kind::lit_int: return "lit_int";
  case token_kind::lit_string: return "lit_string";
  case token_kind::ident: return "ident";
  case token_kind::eof: return "eof";
  case token_kind::comment: return "comment";
  case token_kind::error: return "error";
  case token_kind::kw_void: return "kw_void";
  }
}

std::string token::t_str(const std::string &src) const {
  if (this->m_kind==token_kind::eof) {
    return this->t_kind_str();
  }
  return this->t_kind_str() + " " + src.substr(this->t_span().start_pos(), this->t_span().len());
}

} // namespace dal::core