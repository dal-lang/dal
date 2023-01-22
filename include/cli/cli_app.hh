/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CLI_CLI_APP_HH
#define DAL_CLI_CLI_APP_HH

#include <vector>
#include <fmt/core.hh>
#include "cmd.hh"

namespace dal::cli {

class cli_app {
public:
  cli_app();
  ~cli_app() = default;

  void set_name(const std::string &name);
  void set_version(const std::string &version);
  void set_description(const std::string &description);
  void set_usage(const std::string &usage);
  void set_license(const std::string &license);
  void add_command(const cli_command &command);
  int parse(int argc, char **argv);
private:
  std::string m_app_name;
  std::string m_app_version;
  std::string m_app_desc;
  std::string m_app_usage;
  std::string m_app_license;
  std::map<std::string, cli_command> m_commands;
  std::pair<std::string, std::string> m_additional_commands[3];

  int fallback(int exit_code, const std::string &message = "");
  int version();
  int help();
  int license();
};

}  // namespace dal::cli

#endif //DAL_CLI_CLI_APP_HH
