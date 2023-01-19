/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef _DAL_UTILS_OS_HH_
#define _DAL_UTILS_OS_HH_

#include <string>
#include <vector>
#include <system_error>

namespace dal::utils::os {

    std::string get_cwd();

    std::vector<std::string> split_path(const std::string &path);

    std::string join_path(const std::vector<std::string> &vec);

    std::string get_absolute_path(const std::string &path, std::error_code &ec);

    std::string read_file(const std::string &path, std::error_code &ec);

} // dal::utils::os


#endif //_DAL_UTILS_OS_HH_
