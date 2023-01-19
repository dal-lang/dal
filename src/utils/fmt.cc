/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "fmt.hh"

namespace dal::utils::fmt {
    static bool isTTY() {
        return isatty(fileno(stdout)) || isatty(fileno(stderr));
    }

    void println(const std::string &str) {
        std::cout << str << std::endl;
    }

    void panic(const std::string &str) {
        std::cerr << str << std::endl;
        std::raise(SIGABRT);
    }

    std::string red(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[31m" + str + "\x1b[0m";
    }

    std::string red_bold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[31;1m" + str + "\x1b[0m";
    }

    std::string green(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[32m" + str + "\x1b[0m";
    }

    std::string green_bold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[32;1m" + str + "\x1b[0m";
    }

    std::string yellow(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[33m" + str + "\x1b[0m";
    }

    std::string yellow_bold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[33;1m" + str + "\x1b[0m";
    }

    std::string blue(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[34m" + str + "\x1b[0m";
    }

    std::string blue_bold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[34;1m" + str + "\x1b[0m";
    }

    std::string magenta(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[35m" + str + "\x1b[0m";
    }

    std::string magenta_bold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[35;1m" + str + "\x1b[0m";
    }

    std::string cyan(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[36m" + str + "\x1b[0m";
    }

    std::string cyan_bold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[36;1m" + str + "\x1b[0m";
    }

    std::string pad(const std::string &basic_string, ssize_t usage) {
        if (usage < 0)
            return basic_string;
        if (basic_string.size() >= usage)
            return basic_string;
        return basic_string + std::string(usage - basic_string.size(), ' ');
    }
}