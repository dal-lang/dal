/*
* Copyright (c) 2023 Ade M Ramdani
*
* This file is part of dal, which is MIT licensed.
* See the file "LICENSE" for more information or visit
* https://opensource.org/licenses/MIT for full license details.
*/

#include <fmt/core.hh>

namespace fmt {

void println(const std::string& text) {
  std::cout << text << std::endl;
}

static bool is_tty() {
  return isatty(fileno(stdout)) || isatty(fileno(stderr));
}

std::string red(const std::string& text) {
  if (!is_tty())
    return text;
  return "\x1b[31m" + text + "\x1b[0m";
}

std::string red_bold(const std::string& text) {
  if (!is_tty())
    return text;
  return "\x1b[31;1m" + text + "\x1b[0m";
}

std::string green(const std::string& text) {
  if (!is_tty())
    return text;
  return "\x1b[32m" + text + "\x1b[0m";
}

std::string green_bold(const std::string& text) {
  if (!is_tty())
    return text;
  return "\x1b[32;1m" + text + "\x1b[0m";
}

std::string yellow(const std::string& text) {
  if (!is_tty())
    return text;
  return "\x1b[33m" + text + "\x1b[0m";
}

std::string yellow_bold(const std::string& text) {
  if (!is_tty())
    return text;
  return "\x1b[33;1m" + text + "\x1b[0m";
}

}  // namespace fmt