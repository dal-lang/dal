/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/error.hh>
#include <fmt/core.hh>
#include <vector>

namespace dal::core {

static std::vector<unsigned long> get_line_offsets(const std::string &src) {
  std::vector<unsigned long> offsets;

  offsets.push_back(0);
  for (unsigned long i = 0; i < src.size(); i++) {
    if (src[i]=='\n') {
      offsets.push_back(i + 1);
    }
  }

  return offsets;
}

error::error(const std::string &reason, const span &e_span,
             const std::string &path, const std::string &src) {
  auto start_line = e_span.start_line();
  auto offsets = get_line_offsets(src);
  auto line = src.substr(offsets[start_line - 1], offsets[start_line] -
      offsets[start_line - 1]);
  if (line.back()=='\n') {
    line.pop_back();
  }

  auto num_str = std::to_string(start_line);
  this->m_err +=
      fmt::format("{}: {}\n", fmt::red_bold("error"), fmt::yellow_bold(reason));
  this->m_err += fmt::yellow(
      fmt::format("  --> {}:{}:{}\n", path, start_line, e_span.start_col()));
  this->m_err += fmt::format("{}|\n", std::string(num_str.size() + 2, ' '));
  this->m_err += fmt::format(" {} | {}\n", start_line, fmt::red_bold(line));
  this->m_err += fmt::format(
      "{}{}\n", std::string(e_span.start_col() + num_str.size() + 3, ' '),
      fmt::red_bold("^"));
}

void error::raise(bool need_exit) const {
  fmt::eprintln("{}", this->m_err);
  if (need_exit) {
    exit(1);
  }
}

}  // namespace dal::core