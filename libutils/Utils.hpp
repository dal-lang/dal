/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_UTILS_HPP
#define DAL_UTILS_HPP

#include <string>

class Utils {
public:
    static bool hasPrefix(const std::string &str, const std::string &prefix);
};


#endif //DAL_UTILS_HPP
