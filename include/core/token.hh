/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_TOKEN_HH
#define DAL_CORE_TOKEN_HH

#include <string>
#include "span.hh"

namespace dal::core {

enum class token_kind {
  // Keywords
  kw_let,
  kw_const,
  kw_mut,
  kw_if,
  kw_else,
  kw_fn,
  kw_return,
  kw_pub,
  kw_true,
  kw_false,
  kw_extern,
  kw_import,
  kw_as,
  kw_void,

  // Operators
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
  op_not,
  op_xor,
  op_shl,
  op_shr,
  op_assign,
  op_log_and,
  op_log_or,
  op_log_not,

  // Punctuation
  punct_lparen,
  punct_rparen,
  punct_lbrace,
  punct_rbrace,
  punct_lbracket,
  punct_rbracket,
  punct_comma,
  punct_colon,
  punct_semicolon,
  punct_dot,
  punct_arrow,
  punct_ellipsis,
  punct_at,

  // Literals
  lit_int,
  lit_string,

  // Identifiers
  ident,

  // Misc
  eof,
  comment,
  error,
};

class token {
public:
  token() = default;
  token(token_kind t_kind, span t_span);
  ~token() = default;

  [[nodiscard]] token_kind t_kind() const;
  [[nodiscard]] span t_span() const;
  [[nodiscard]] std::string t_kind_str() const;
  [[nodiscard]] std::string t_str(const std::string &src) const;
private:
  token_kind m_kind = token_kind::eof;
  span m_span;
};

} // namespace dal::core

#endif //DAL_CORE_TOKEN_HH
