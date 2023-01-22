/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_TOKENIZER_HH
#define DAL_CORE_TOKENIZER_HH

#include <vector>
#include <string>
#include <functional>
#include "token.hh"
#include "error.hh"

namespace dal::core {

class tokenizer {
public:
  tokenizer() = default;
  ~tokenizer() = default;

  std::vector<token> tokenize(const std::string &str);
  [[nodiscard]] bool has_error() const;
  void print_error();
private:
  std::string m_str;
  std::vector<char> m_src;
  std::vector<error> m_errors;
  span m_span;

  token next_token();
  char bump();
  bool is_eof();
  char first();
  char second();
  span eat_span();
  span t_span();
  void eat_while(const std::function<bool(char)> &f);
  std::string t_raw();
};

} // namespace dal::core

#endif //DAL_CORE_TOKENIZER_HH
