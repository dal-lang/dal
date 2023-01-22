/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/error.hh>
#include <stdexcept>
#include <fmt/core.hh>

namespace dal::core {

error::error(const std::string &reason, const span &e_span) {
  this->m_reason = reason;
  this->m_span = e_span;
}

void error::raise(const std::string &src) const {
  auto start_pos = this->m_span.start_pos();
  auto end_pos = this->m_span.end_pos();
  auto start_line = this->m_span.start_line();
  auto end_line = this->m_span.end_line();

  std::string msg = fmt::format("{}: {}", fmt::red_bold("error"), this->m_reason);
  if (start_line==end_line) {
    msg += fmt::format(" ({}:{}-{})", start_line, start_pos, end_pos);
  } else {
    msg += fmt::format(" ({}:{}-{}:{})", start_line, start_pos, end_line, end_pos);
  }

  msg += "\n";
  msg += fmt::format("{} | {}\n", start_line, src.substr(start_pos, end_pos - start_pos));
  msg += fmt::format("{} | {}\n", start_line, std::string(start_pos, ' ') + std::string(end_pos - start_pos, '^'));

  throw std::runtime_error(msg);
}

} // namespace dal::core