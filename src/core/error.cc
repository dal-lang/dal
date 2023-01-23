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

error::error(const std::string& reason, const span& e_span,
             const std::string& path) {
  this->m_reason = reason;
  this->m_path = path;
  this->m_span = e_span;
}

static std::vector<int> get_line_offsets(const std::string& src) {
  std::vector<int> offsets;

  offsets.push_back(0);
  for (auto i = 0; i < src.size(); i++) {
    if (src[i] == '\n') {
      offsets.push_back(i + 1);
    }
  }

  return offsets;
}

void error::raise(const std::string& src, bool need_exit) const {
  auto start_line = this->m_span.start_line();
  auto offsets = get_line_offsets(src);
  auto line = src.substr(offsets[start_line],
                         offsets[start_line + 1] - offsets[start_line]);
  if (line.back() == '\n') {
    line.pop_back();
  }

  auto num_str = std::to_string(start_line + 1);
  fmt::eprintln("{}: {}", fmt::red_bold("error"),
                fmt::yellow_bold(this->m_reason));
  fmt::eprintln(
      "{}", fmt::yellow(fmt::format("  --> {}:{}:{}", this->m_path,
                                    start_line + 1, this->m_span.start_col())));
  fmt::eprintln("{}|", std::string(num_str.size() + 2, ' '));
  fmt::eprintln(" {} | {}", start_line + 1, fmt::red_bold(line));
  fmt::eprintln("{}{}",
                std::string(this->m_span.start_col() + num_str.size() + 3, ' '),
                fmt::red_bold("^"));

  if (need_exit) {
    exit(1);
  }
}

}  // namespace dal::core