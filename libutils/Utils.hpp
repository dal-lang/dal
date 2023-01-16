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

/**
 * @brief Class for utilities
 * @details This class contains some utilities that can be used in the
 *         program.
 */
class Utils {
public:
    /**
     * @brief Check if a string has a prefix
     *
     * @param str String to check
     * @param prefix Prefix to check
     * @return true String has prefix
     */
    static bool hasPrefix(const std::string &str, const std::string &prefix);
};


#endif //DAL_UTILS_HPP
