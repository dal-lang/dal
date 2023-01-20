/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CLI_CTX_HH
#define DAL_CLI_CTX_HH

#include <map>
#include "arg.hh"

namespace dal::cli {

class context {
public:
  explicit context(const std::map<std::string, cli_arg *> &m_args);
  ~context() = default;

  std::string get_string(const std::string &name);
  int get_int(const std::string &name);
  bool get_bool(const std::string &name);
private:
  std::map<std::string, cli_arg *> m_args;
};

} // namespace dal::cli

#endif //DAL_CLI_CTX_HH
