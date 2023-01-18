/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <csignal>
#include <filesystem>
#include <fstream>
#include "Os.hpp"

std::string Os::getCwd() {
    char cwd[1024];
    if (getcwd(cwd, sizeof(cwd)) != nullptr) {
        return cwd;
    } else {
        return "";
    }
}

std::string Os::readFile(const std::string &path, std::error_code &ec) {
    auto absPath = getAbsolutePath(path, ec);
    if (ec) {
        return "";
    }

    // Check if the file exists and is a regular file.
    if (!std::filesystem::exists(absPath) || !std::filesystem::is_regular_file(absPath)) {
        ec = std::make_error_code(std::errc::no_such_file_or_directory);
        return "";
    }

    std::ifstream file(absPath);
    if (!file.is_open()) {
        ec = std::make_error_code(std::errc::no_such_file_or_directory);
        return "";
    }

    std::string content((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
    return content;
}

std::vector<std::string> Os::splitPath(const std::string &path) {
    std::vector<std::string> vec;
    std::string temp;
    for (auto c : path) {
        if (c == '/') {
            vec.push_back(temp);
            temp = "";
        } else {
            temp += c;
        }
    }
    vec.push_back(temp);
    return vec;
}

std::string Os::joinPath(const std::vector<std::string> &vec) {
    std::string path;
    for (auto &s : vec) {
        path += s + "/";
    }
    return path;
}

std::string Os::getAbsolutePath(const std::string &path, std::error_code &ec) {
    // This will convert the path to absolute path.
    // Example:
    // cwd: /home/xxx
    // path: ../yyy
    // result: /home/yyy
    std::filesystem::path p = std::filesystem::canonical(path, ec);
    if (ec) {
        return "";
    }
    return p.string();
}