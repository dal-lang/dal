/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef _DAL_UTILS_FMT_HH_
#define _DAL_UTILS_FMT_HH_

#include <string>
#include <iostream>
#include <csignal>

namespace dal::utils::fmt {

    template<typename... T>
    std::string format(const std::string &fmt, T &&...args) {
        auto result = fmt;
        auto args_list = {std::forward<T>(args)...};
        for (const auto &arg: args_list) {
            auto pos = result.find("{}");
            if (pos == std::string::npos) {
                break;
            }
            if constexpr (std::is_same_v<std::string, std::decay_t<decltype(arg)>>) {
                result.replace(pos, 2, arg);
            } else if constexpr (std::is_same_v<const std::string &, std::decay_t<decltype(arg)>>) {
                result.replace(pos, 2, arg);
            } else if constexpr (std::is_same_v<char const *, std::decay_t<decltype(arg)>>) {
                result.replace(pos, 2, arg);
            } else if constexpr (std::is_same_v<char *, std::decay_t<decltype(arg)>>) {
                result.replace(pos, 2, arg);
            } else if constexpr (std::is_same_v<const char *, std::decay_t<decltype(arg)>>) {
                result.replace(pos, 2, arg);
            } else if constexpr (std::is_same_v<char const &, std::decay_t<decltype(arg)>>) {
                result.replace(pos, 2, std::string(1, arg));
            } else if constexpr (std::is_same_v<const char *&, std::decay_t<decltype(arg)>>) {
                result.replace(pos, 2, arg);
            } else {
                result.replace(pos, 2, std::to_string(arg));
            }
        }
        return result;
    }

    template<typename... Args>
    void println(const std::string &fmt, Args... args) {
        std::cout << format(fmt, args...) << std::endl;
    }

    void println(const std::string &str);

    template<typename... Args>
    void print(const std::string &fmt, Args... args) {
        std::cout << format(fmt, args...);
    }

    void print(const std::string &str);

    template<typename... Args>
    void eprintln(const std::string &fmt, Args... args) {
        std::cerr << format(fmt, args...) << std::endl;
    }

    template<typename... Args>
    void panic(const std::string &fmt, Args... args) {
        std::cerr << format(fmt, args...) << std::endl;
        std::raise(SIGABRT);
    }

    void panic(const std::string &str);

    std::string red(const std::string &str);

    std::string red_bold(const std::string &str);

    std::string green(const std::string &str);

    std::string green_bold(const std::string &str);

    std::string yellow(const std::string &str);

    std::string yellow_bold(const std::string &str);

    std::string blue(const std::string &str);

    std::string blue_bold(const std::string &str);

    std::string magenta(const std::string &str);

    std::string magenta_bold(const std::string &str);

    std::string cyan(const std::string &str);

    std::string cyan_bold(const std::string &str);

    std::string pad(const std::string &basic_string, ssize_t usage);
}

#endif //_DAL_UTILS_FMT_HH_
