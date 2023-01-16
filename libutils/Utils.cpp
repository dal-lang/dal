/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Utils.hpp"

bool Utils::hasPrefix(const std::string &str, const std::string &prefix) {
    return std::mismatch(prefix.begin(), prefix.end(), str.begin(), str.end()).first == prefix.end();
}
