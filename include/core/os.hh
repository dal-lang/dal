/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_OS_HH
#define DAL_CORE_OS_HH

#include <string>
#include <system_error>

namespace dal::core::os {

std::string get_cwd();
std::string to_abs_path(const std::string &path, std::error_code &ec);
std::string read_file(const std::string &path, std::error_code &ec);
void split_path(const std::string &path, std::string &dir, std::string &file, std::error_code &ec);
std::string join_path(const std::string &dir, const std::string &file, std::error_code &ec);
bool is_file(const std::string &path, std::error_code &ec);

} // namespace dal::core::os

#endif //DAL_CORE_OS_HH
