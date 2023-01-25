/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <algorithm>
#include <cli/cli_app.hh>
#include <utility>

namespace dal::cli {

void cli_command::set_name(const std::string& name) {
  this->m_name = name;
}

void cli_command::set_description(const std::string& description) {
  this->m_desc = description;
}

void cli_command::set_usage(const std::string& usage) {
  this->m_usage = usage;
}

void cli_command::add_arg(std::unique_ptr<cli_arg> arg) {
  this->m_args.insert({arg->get_name(), std::move(arg)});
}

void cli_command::set_handler(std::function<int(context)> handler) {
  this->m_handler = std::move(handler);
}

std::string cli_command::get_name() const {
  return this->m_name;
}

std::string cli_command::get_description() const {
  return this->m_desc;
}

std::string cli_command::get_usage() const {
  return this->m_usage;
}

static bool has_prefix(const std::string& str, const std::string& prefix) {
  return std::mismatch(prefix.begin(), prefix.end(), str.begin()).first ==
         prefix.end();
}

int cli_command::parse(std::vector<std::string> args) {
  auto err_label = fmt::red_bold("error");
  std::map<std::string, bool> supplied;
  for (unsigned long i = 0; i < args.size(); i++) {
    auto arg = args[i];
    if (has_prefix(arg, "--"))
      arg = arg.substr(2);

    auto flag = this->m_args.find(arg);
    if (flag->first.empty()) {
      fmt::eprintln("{}: Unknown options `--{}`", err_label, arg);
      return this->help(1);
    }

    switch (flag->second->get_type()) {
      case cli_arg_type::string: {
        if (i + 1 >= args.size()) {
          fmt::eprintln("{}: Missing value for argument `--{}`", err_label,
                        arg);
          return 1;
        }
        i++;
        flag->second->set_value(args[i]);
        supplied[flag->second->get_name()] = true;
        continue;
      }
      case cli_arg_type::number: {
        if (i + 1 >= args.size()) {
          fmt::eprintln("{}: Missing value for argument `--{}`", err_label,
                        arg);
          return 1;
        }
        i++;
        // because we disable exceptions, we need to manually check if the value is a number.
        for (auto c : args[i]) {
          if (!std::isdigit(c)) {
            fmt::eprintln("{}: Invalid value for argument `--{}`", err_label,
                          arg);
            return 1;
          }
        }
        flag->second->set_value(std::stoi(args[i]));
        supplied[flag->second->get_name()] = true;
        continue;
      }
      case cli_arg_type::boolean: {
        flag->second->set_value(true);
        supplied[flag->second->get_name()] = true;
        continue;
      }
    }
  }

  // verify all arguments.
  for (const auto& m_arg : this->m_args) {
    auto arg = this->m_args.at(m_arg.first);
    switch (arg->get_type()) {
      case cli_arg_type::string: {
        if (supplied[arg->get_name()] ||
            !arg->get_default_value<std::string>().empty()) {
          continue;
        }
        if (arg->is_required() &&
            arg->get_default_value<std::string>().empty()) {
          fmt::eprintln(
              "{}: Argument `--{}` marked as required, but no value was "
              "supplied",
              err_label, arg->get_name());
          return 1;
        }
        break;
      }
      case cli_arg_type::number: {
        if (supplied[arg->get_name()] || arg->get_default_value<int>() != 0) {
          continue;
        }
        if (arg->is_required()) {
          fmt::eprintln(
              "{}: Argument `--{}` marked as required, but no value was "
              "supplied",
              err_label, arg->get_name());
          return 1;
        }
        break;
      }
      case cli_arg_type::boolean: {
        if (supplied[arg->get_name()] || arg->get_default_value<bool>()) {
          continue;
        }
        if (arg->is_required()) {
          fmt::eprintln(
              "{}: Argument `--{}` marked as required, but no value was "
              "supplied",
              err_label, arg->get_name());
          return 1;
        }
        break;
      }
    }
  }

  context ctx(this->m_args);
  return this->m_handler(ctx);
}

int cli_command::help(int exit_code) {
  auto stream = exit_code == 0 ? stdout : stderr;
  fmt::print(stream, "{}: {}\n", fmt::green_bold("Usage"), this->m_usage);

  if (!this->m_args.empty())
    fmt::print(stream, "\n{}:\n", fmt::green_bold("Options"));

  unsigned long longest_name = 8;
  std::string keys[this->m_args.size()];
  int i = 0;
  for (auto& arg : this->m_args) {
    if (arg.first.length() > longest_name)
      longest_name = arg.first.length();
    keys[i] = arg.first;
    i++;
  }

  std::sort(keys, keys + this->m_args.size());

  for (auto& key : keys) {
    auto arg = this->m_args[key];
    auto name = arg->get_name();
    auto desc = arg->get_description();
    auto usage = arg->get_usage();
    auto required = arg->is_required();
    std::string default_value;

    if (arg->get_type() == cli_arg_type::string) {
      default_value = arg->get_default_value<std::string>();
    } else if (arg->get_type() == cli_arg_type::number) {
      default_value = std::to_string(arg->get_default_value<int>());
    } else if (arg->get_type() == cli_arg_type::boolean) {
      default_value = arg->get_default_value<bool>() ? "true" : "false";
    }

    std::string usage_str = "Usage";
    std::string required_str = "Required";
    std::string default_value_str = "Default";

    fmt::print(stream, "  --{}{}{}{}{}\n", name,
               std::string(longest_name - name.length(), ' '), fmt::yellow(":"),
               std::string(2, ' '), desc);
    fmt::print(stream, "    {}{}   {}\n", fmt::yellow(usage_str),
               std::string(longest_name - usage_str.length(), ' '), usage);
    if (required)
      fmt::print(stream, "    {}{}   {}\n", fmt::yellow(required_str),
                 std::string(longest_name - required_str.length(), ' '),
                 required);
    if (!default_value.empty())
      fmt::print(stream, "    {}{}   {}\n", fmt::yellow(default_value_str),
                 std::string(longest_name - default_value_str.length(), ' '),
                 default_value);
  }

  return exit_code;
}

}  // namespace dal::cli