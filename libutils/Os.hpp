/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_OS_HPP
#define DAL_OS_HPP

#include <string>
#include <vector>
#include <system_error>

class Os {
public:
    static std::string getCwd();

    static std::vector<std::string> splitPath(const std::string &path);

    static std::string joinPath(const std::vector<std::string> &vec);

    static std::string getAbsolutePath(const std::string &path, std::error_code &ec);

    static std::string readFile(const std::string &path, std::error_code &ec);
};


#endif //DAL_OS_HPP
