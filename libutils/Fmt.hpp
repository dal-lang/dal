/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_FMT_HPP
#define DAL_FMT_HPP

#include <cstdio>
#include <csignal>

class Fmt {
private:
    template<typename T, typename... Args>
    struct is_type : std::true_type {
    };

    static bool isTTY() {
        return isatty(fileno(stdout)) || isatty(fileno(stderr));
    }

public:
    template<typename... Args>
    static void println(const std::string &fmt, Args... args) {
        if constexpr (is_type<std::string, Args...>::value) {
            printf(fmt.c_str(), args.c_str()...);
        } else {
            printf(fmt.c_str(), args...);
        }
        printf("\n");
    }

    static void println(const std::string &str) {
        printf("%s\n", str.c_str());
    }

    template<typename... Args>
    static void print(const std::string &fmt, Args... args) {
        if constexpr (is_type<std::string, Args...>::value) {
            printf(fmt.c_str(), args.c_str()...);
        } else {
            printf(fmt.c_str(), args...);
        }
    }

    static void print(const std::string &str) {
        printf("%s", str.c_str());
    }

    template<typename... Args>
    static void eprintln(const std::string &fmt, Args... args) {
        if constexpr (is_type<std::string, Args...>::value) {
            fprintf(stderr, fmt.c_str(), args.c_str()...);
        } else {
            fprintf(stderr, fmt.c_str(), args...);
        }
        fprintf(stderr, "\n");
    }

    template<typename... Args>
    static void panic(const std::string &fmt, Args... args) {
        Fmt::eprintln(fmt, args...);
        std::raise(SIGABRT);
    }

    static void panic(const std::string &str) {
        fprintf(stderr, "%s\n", str.c_str());
        exit(1);
    }

    template<typename... Args>
    static std::string format(const std::string &fmt, Args... args) {
        char buffer[1024];
        if constexpr (is_type<std::string, Args...>::value) {
            sprintf(buffer, fmt.c_str(), args.c_str()...);
        } else {
            sprintf(buffer, fmt.c_str(), args...);
        }
        return buffer;
    }

    static std::string red(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[31m" + str + "\x1b[0m";
    }

    static std::string redBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[31;1m" + str + "\x1b[0m";
    }

    static std::string green(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[32m" + str + "\x1b[0m";
    }

    static std::string greenBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[32;1m" + str + "\x1b[0m";
    }

    static std::string yellow(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[33m" + str + "\x1b[0m";
    }

    static std::string yellowBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[33;1m" + str + "\x1b[0m";
    }

    static std::string blue(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[34m" + str + "\x1b[0m";
    }

    static std::string blueBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[34;1m" + str + "\x1b[0m";
    }

    static std::string magenta(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[35m" + str + "\x1b[0m";
    }

    static std::string magentaBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[35;1m" + str + "\x1b[0m";
    }

    static std::string cyan(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[36m" + str + "\x1b[0m";
    }

    static std::string cyanBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[36;1m" + str + "\x1b[0m";
    }

    static std::string pad(std::string basicString, ssize_t usage) {
        if (usage < 0)
            return basicString;
        if (basicString.size() >= usage)
            return basicString;
        return basicString + std::string(usage - basicString.size(), ' ');
    }
};


#endif //DAL_FMT_HPP
