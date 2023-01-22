/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <cli/cli_app.hh>
#include "cli/arg.hh"

namespace dal::cli {

void cli_arg::set_name(const std::string &name) {
  this->m_name = name;
}

void cli_arg::set_description(const std::string &description) {
  this->m_desc = description;
}

void cli_arg::set_usage(const std::string &usage) {
  this->m_usage = usage;
}

void cli_arg::set_required(bool required) {
  this->m_required = required;
}

void cli_arg::set_default_value(const std::string &default_value) {
  this->m_str_default_value = default_value;
}

void cli_arg::set_default_value(int default_value) {
  this->m_int_default_value = default_value;
}

void cli_arg::set_default_value(bool default_value) {
  this->m_bool_default_value = default_value;
}

void cli_arg::set_value(const std::string &value) {
  this->m_str_value = value;
}

void cli_arg::set_value(int value) {
  this->m_int_default_value = value;
}

void cli_arg::set_value(bool value) {
  this->m_bool_default_value = value;
}

std::string cli_arg::get_name() const {
  return this->m_name;
}

std::string cli_arg::get_description() const {
  return this->m_desc;
}

std::string cli_arg::get_usage() const {
  return this->m_usage;
}

bool cli_arg::is_required() const {
  return this->m_required;
}

cli_arg_type cli_arg::get_type() const {
  return this->m_type;
}

template<>
std::string cli_arg::get_default_value<std::string>() const {
  return this->m_str_default_value;
}

template<>
int cli_arg::get_default_value<int>() const {
  return this->m_int_default_value;
}

template<>
bool cli_arg::get_default_value<bool>() const {
  return this->m_bool_default_value;
}

template<>
std::string cli_arg::get_value<std::string>() const {
  return this->m_str_value;
}

template<>
int cli_arg::get_value<int>() const {
  return this->m_int_default_value;
}

template<>
bool cli_arg::get_value<bool>() const {
  return this->m_bool_default_value;
}

arg_builder &arg_builder::set_name(const std::string &name) {
  this->m_name = name;
  return *this;
}

arg_builder &arg_builder::set_description(const std::string &description) {
  this->m_desc = description;
  return *this;
}

arg_builder &arg_builder::set_usage(const std::string &usage) {
  this->m_usage = usage;
  return *this;
}

arg_builder &arg_builder::set_required(bool required) {
  this->m_required = required;
  return *this;
}

arg_builder &arg_builder::set_str_default(const std::string &default_value) {
  this->m_str_default_value = default_value;
  return *this;
}

arg_builder &arg_builder::set_int_default(int default_value) {
  this->m_int_default_value = default_value;
  return *this;
}

arg_builder &arg_builder::set_bool_default(bool default_value) {
  this->m_bool_default_value = default_value;
  return *this;
}

cli_arg *arg_builder::build_string() {
  auto arg = new cli_arg(cli_arg_type::string);
  arg->set_name(this->m_name);
  arg->set_description(this->m_desc);
  arg->set_usage(this->m_usage);
  arg->set_required(this->m_required);
  arg->set_default_value(this->m_str_default_value);

  // reset builder
  this->m_name = "";
  this->m_desc = "";
  this->m_usage = "";
  this->m_required = false;
  this->m_str_default_value = "";
  this->m_int_default_value = 0;
  this->m_bool_default_value = false;

  return dynamic_cast<cli_arg *>(arg);
}

cli_arg *arg_builder::build_int() {
  auto arg = new cli_arg(cli_arg_type::number);
  arg->set_name(this->m_name);
  arg->set_description(this->m_desc);
  arg->set_usage(this->m_usage);
  arg->set_required(this->m_required);
  arg->set_default_value(this->m_int_default_value);

  // reset builder
  this->m_name = "";
  this->m_desc = "";
  this->m_usage = "";
  this->m_required = false;
  this->m_str_default_value = "";
  this->m_int_default_value = 0;
  this->m_bool_default_value = false;

  return dynamic_cast<cli_arg *>(arg);
}

cli_arg *arg_builder::build_bool() {
  auto arg = new cli_arg(cli_arg_type::boolean);
  arg->set_name(this->m_name);
  arg->set_description(this->m_desc);
  arg->set_usage(this->m_usage);
  arg->set_required(this->m_required);
  arg->set_default_value(this->m_bool_default_value);

  // reset builder
  this->m_name = "";
  this->m_desc = "";
  this->m_usage = "";
  this->m_required = false;
  this->m_str_default_value = "";
  this->m_int_default_value = 0;
  this->m_bool_default_value = false;

  return dynamic_cast<cli_arg *>(arg);
}

} // namespace dal::cli