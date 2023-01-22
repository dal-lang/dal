/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <cli/cli_app.hh>

namespace dal::cli {

context::context(const std::map<std::string, cli_arg *> &m_args) {
  this->m_args = m_args;
}

std::string context::get_string(const std::string &name) {
  std::string result;
  if (this->m_args.find(name)!=this->m_args.end()) {
    auto arg = this->m_args.at(name);
    if (arg->get_type()==cli_arg_type::string) {
      if (!arg->get_value<std::string>().empty()) {
        result = arg->get_value<std::string>();
      } else {
        result = arg->get_default_value<std::string>();
      }
    }
  }
  return result;
}

int context::get_int(const std::string &name) {
  int result = 0;
  if (this->m_args.find(name)!=this->m_args.end()) {
    auto arg = this->m_args.at(name);
    if (arg->get_type()==cli_arg_type::number) {
      if (arg->get_value<int>()!=0) {
        result = arg->get_value<int>();
      } else {
        result = arg->get_default_value<int>();
      }
    }
  }
  return result;
}

bool context::get_bool(const std::string &name) {
  bool result = false;
  if (this->m_args.find(name)!=this->m_args.end()) {
    auto arg = this->m_args.at(name);
    if (arg->get_type()==cli_arg_type::boolean) {
      if (arg->get_value<bool>()) {
        result = arg->get_value<bool>();
      } else {
        result = arg->get_default_value<bool>();
      }
    }
  }
  return result;
}

} // namespace dal::cli