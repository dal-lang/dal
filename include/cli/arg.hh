/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CLI_ARG_HH
#define DAL_CLI_ARG_HH

#include <memory>
#include <string>

namespace dal::cli {

enum class cli_arg_type {
  string,
  number,
  boolean,
};

class cli_arg {
public:
  explicit cli_arg(cli_arg_type type) : m_type(type) {}
  ~cli_arg() = default;

  void set_name(const std::string &name);
  void set_description(const std::string &description);
  void set_usage(const std::string &usage);
  void set_required(bool required);

  void set_default_value(const std::string &default_value);
  void set_default_value(int default_value);
  void set_default_value(bool default_value);

  void set_value(const std::string &value);
  void set_value(int value);
  void set_value(bool value);

  [[nodiscard]] std::string get_name() const;
  [[nodiscard]] std::string get_description() const;
  [[nodiscard]] std::string get_usage() const;
  [[nodiscard]] bool is_required() const;
  [[nodiscard]] cli_arg_type get_type() const;

  template<typename T>
  T get_default_value() const;

  template<typename T>
  T get_value() const;
private:
  std::string m_name;
  std::string m_desc;
  std::string m_usage;
  cli_arg_type m_type;
  bool m_required = false;
  std::string m_str_default_value;
  int m_int_default_value = 0;
  bool m_bool_default_value = false;
  std::string m_str_value;
  int m_int_value = 0;
  bool m_bool_value = false;
};

class arg_builder {
public:
  arg_builder() = default;
  ~arg_builder() = default;

  arg_builder &set_name(const std::string &name);
  arg_builder &set_description(const std::string &description);
  arg_builder &set_usage(const std::string &usage);
  arg_builder &set_required(bool required);

  arg_builder &set_str_default(const std::string &default_value);
  arg_builder &set_int_default(int default_value);
  arg_builder &set_bool_default(bool default_value);

  std::unique_ptr<cli_arg> build_string();
  std::unique_ptr<cli_arg> build_int();
  std::unique_ptr<cli_arg> build_bool();
private:
  std::string m_name;
  std::string m_desc;
  std::string m_usage;
  bool m_required = false;
  std::string m_str_default_value;
  int m_int_default_value = 0;
  bool m_bool_default_value = false;
};

} // namespace dal::cli

#endif //DAL_CLI_ARG_HH
