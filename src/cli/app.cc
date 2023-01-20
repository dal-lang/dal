/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <cli/cli_app.hh>
#include <algorithm>

namespace dal::cli {

cli_app::cli_app() {
  this->m_additional_commands[0] = std::make_pair("help", "Show help message");
  this->m_additional_commands[1] = std::make_pair("license", "Show license information");
  this->m_additional_commands[2] = std::make_pair("version", "Show version information");
}

void cli_app::set_name(const std::string &name) {
  this->m_app_name = name;
}

void cli_app::set_version(const std::string &version) {
  this->m_app_version = version;
}

void cli_app::set_description(const std::string &description) {
  this->m_app_desc = description;
}

void cli_app::set_usage(const std::string &usage) {
  this->m_app_usage = usage;
}

void cli_app::set_license(const std::string &license) {
  this->m_app_license = license;
}

void cli_app::add_command(const cli_command &command) {
  this->m_commands.insert({command.get_name(), command});
}

int cli_app::parse(int argc, char **argv) {
  std::vector<std::string> args;
  std::string cmd;
  for (int i = 1; i < argc; i++) {
    if (i==1) {
      cmd = argv[i];
      continue;
    }
    args.emplace_back(argv[i]);
  }

  if (argc==1)
    return this->fallback(1, "No arguments provided");

  if (cmd=="help") {
    if (!args[0].empty()) {
      auto command = &this->m_commands[args[0]];
      if (command)
        return command->help();
      else
        return this->fallback(1, "Command not found");
    }
    return this->help();
  } else if (cmd=="license") {
    return this->license();
  } else if (cmd=="version") {
    return this->version();
  }

  for (auto &command : this->m_commands) {
    auto m_cmd = &command.second;
    if (cmd==m_cmd->get_name()) {
      return m_cmd->parse(args);
    }
  }

  return this->fallback(1, fmt::format("Command '{}' not found", args[0]));
}

int cli_app::fallback(int exit_code, const std::string &message) {
  auto stream = (exit_code==0) ? stdout : stderr;

  if (!message.empty())
    fmt::print(stream, "{}: {}\n\n", fmt::red_bold("error"), message);
  fmt::print(stream, "{}: {}\n", fmt::green_bold("Usage"), this->m_app_usage);

  if (!this->m_commands.empty()) {
    fmt::print(stream, "{}:\n", fmt::green_bold("Commands"));

    unsigned long longest_name = 0;
    std::string command_names[this->m_commands.size()];
    int i = 0;
    for (const auto &command : this->m_commands) {
      auto cmd = &command.second;
      command_names[i] = cmd->get_name();
      if (cmd->get_name().length() > longest_name)
        longest_name = cmd->get_name().length();
      i++;
    }

    std::sort(command_names, command_names + this->m_commands.size());

    for (const auto &command : this->m_commands) {
      auto cmd = &command.second;
      fmt::print(stream, "  {}{}{}{}{}\n",
                 fmt::green_bold(cmd->get_name()),
                 std::string(longest_name - cmd->get_name().length(), ' '),
                 "  ",
                 cmd->get_description(),
                 "");
    }
  }

  fmt::print(stream, "{}:\n", fmt::green_bold("Additional commands"));
  unsigned long longest_name = 0;
  for (const auto &command : this->m_additional_commands) {
    if (command.first.length() > longest_name)
      longest_name = command.first.length();
  }

  for (const auto &command : this->m_additional_commands) {
    fmt::print(stream, "  {}{}{}{}{}\n",
               fmt::green_bold(command.first),
               std::string(longest_name - command.first.length(), ' '),
               "  ",
               command.second,
               "");
  }

  return exit_code;
}

int cli_app::help() {
  fmt::println("{}\n", this->m_app_desc);
  this->fallback(0, "");
  return 0;
}

int cli_app::license() {
  fmt::println("{}\n", this->m_app_license);
  return 0;
}

int cli_app::version() {
  fmt::println("{}: {}\n", this->m_app_name, this->m_app_version);
  return 0;
}

}  // namespace dal::cli
