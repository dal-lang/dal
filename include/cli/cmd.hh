/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CLI_CMD_HH
#define DAL_CLI_CMD_HH

#include "ctx.hh"

namespace dal::cli {

typedef int(*command_handler)(context ctx);

class cli_command {
public:
  cli_command() = default;
  ~cli_command() = default;

  void set_name(const std::string &name);
  void set_description(const std::string &description);
  void set_usage(const std::string &usage);
  void add_arg(cli_arg *arg);
  void set_handler(command_handler handler);

  [[nodiscard]] std::string get_name() const;
  [[nodiscard]] std::string get_description() const;
  [[nodiscard]] std::string get_usage() const;

  int parse(std::vector<std::string> args);
  int help(int exit_code = 0);
private:
  std::string m_name;
  std::string m_desc;
  std::string m_usage;
  std::map<std::string, cli_arg *> m_args;
  command_handler m_handler = nullptr;
};

} // namespace dal::cli

#endif //DAL_CLI_CMD_HH
