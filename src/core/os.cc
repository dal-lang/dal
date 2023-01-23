/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/os.hh>
#include <filesystem>
#include <fstream>
#include <unistd.h>

namespace dal::core::os {

std::string get_cwd() {
  char *cwd = getcwd(nullptr, 0);
  std::string cwd_str(cwd);
  free(cwd);
  return cwd_str;
}

std::string to_abs_path(const std::string &path, std::error_code &ec) {
  char *abs_path = realpath(path.c_str(), nullptr);
  if (abs_path==nullptr) {
    ec = std::error_code(errno, std::system_category());
    return "";
  }
  std::string abs_path_str(abs_path);
  free(abs_path);
  return abs_path_str;
}

std::string read_file(const std::string &path, std::error_code &ec) {
  auto abs = to_abs_path(path, ec);
  if (ec) {
    return "";
  }

  if (!std::filesystem::exists(abs)) {
    ec = std::error_code(ENOENT, std::system_category());
    return "";
  }

  if (!std::filesystem::is_regular_file(abs)) {
    ec = std::error_code(EISDIR, std::system_category());
    return "";
  }

  std::ifstream file(abs);
  if (!file.is_open()) {
    ec = std::error_code(errno, std::system_category());
    return "";
  }

  std::string content((std::istreambuf_iterator<char>(file)),
                      std::istreambuf_iterator<char>());
  return content;
}

void split_path(const std::string &path, std::string &dir, std::string &file, std::error_code &ec) {
  auto abs = to_abs_path(path, ec);
  if (ec) {
    return;
  }

  if (!std::filesystem::exists(abs)) {
    ec = std::error_code(ENOENT, std::system_category());
    return;
  }

  if (!std::filesystem::is_regular_file(abs)) {
    ec = std::error_code(EISDIR, std::system_category());
    return;
  }

  auto p = std::filesystem::path(abs);
  dir = p.parent_path();
  file = p.filename();
}

} // namespace dal::core::os