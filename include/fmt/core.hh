/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_FMT_CORE_HH
#define DAL_FMT_CORE_HH

#include <unistd.h>
#include "format.hh"

namespace fmt {

template <typename... T>
void println(const std::string& fmt, T... args) {
  std::cout << fmt::format(fmt, args...) << std::endl;
}

void println(const std::string& text);

template <typename... T>
void eprintln(const std::string& fmt, T... args) {
  std::cerr << fmt::format(fmt, args...) << std::endl;
}

void eprintln(const std::string& text);

template <typename... T>
void print(FILE* stream, const std::string& fmt, T... args) {
  if (stream == stdout) {
    std::cout << fmt::format(fmt, args...);
  } else if (stream == stderr) {
    std::cerr << fmt::format(fmt, args...);
  }
}

template <typename... T>
void print(std::ostream& stream, const std::string& fmt, T... args) {
  stream << fmt::format(fmt, args...);
}

template <typename... T>
[[noreturn]] void panic(const std::string& fmt, T... args) {
  eprintln(fmt, args...);
  std::exit(1);
}

std::string red(const std::string& text);
std::string red_bold(const std::string& text);
std::string green(const std::string& text);
std::string green_bold(const std::string& text);
std::string yellow(const std::string& text);
std::string yellow_bold(const std::string& text);

}  // namespace fmt

#endif  //DAL_FMT_CORE_HH
