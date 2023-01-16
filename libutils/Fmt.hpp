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

/**
 * @brief Class for formatting strings
 */
class Fmt {
private:
    /**
    * @brief Variadic template checker.
    * @details This struct is used to check if the variadic template has type T.
    */
    template<typename T, typename... Args>
    struct is_type : std::true_type {
    };

    /**
     * @brief Check if stdout or stderr is a tty.
     *
     * @return true Stdout or stderr is a tty.
     */
    static bool isTTY() {
        return isatty(fileno(stdout)) || isatty(fileno(stderr));
    }

public:
    /**
     * @brief Print a string with line break
     *
     * @param fmt string to print
     * @param args arguments to print
     */
    template<typename... Args>
    static void println(const std::string &fmt, Args... args) {
        // If args has type std::string, we need to convert it to const char *
        // so we can use printf

        // check if args has type std::string
        if constexpr (is_type<std::string, Args...>::value) {
            // convert std::string to const char *
            printf(fmt.c_str(), args.c_str()...);
        } else {
            printf(fmt.c_str(), args...);
        }
        printf("\n");
    }

    /**
     * @brief Print a string with line break
     *
     * @param str string to print
     */
    static void println(const std::string &str) {
        printf("%s\n", str.c_str());
    }

    /**
     * @brief Print a string without line break
     *
     * @param fmt string to print
     * @param args arguments to print
     */
    template<typename... Args>
    static void print(const std::string &fmt, Args... args) {
        // If args has type std::string, we need to convert it to const char *
        // so we can use printf

        // check if args has type std::string
        if constexpr (is_type<std::string, Args...>::value) {
            // convert std::string to const char *
            printf(fmt.c_str(), args.c_str()...);
        } else {
            printf(fmt.c_str(), args...);
        }
    }

    /**
     * @brief Print a string without line break
     *
     * @param str string to print
     */
    static void print(const std::string &str) {
        printf("%s", str.c_str());
    }

    /**
     * @brief Same as println but print to stderr
     *
     * @param fmt string to print
     * @param args arguments to print
     */
    template<typename... Args>
    static void eprintln(const std::string &fmt, Args... args) {
        fprintf(stderr, fmt.c_str(), args...);
        fprintf(stderr, "\n");
    }

    /**
     * @brief Panic the program with a message
     *
     * @param fmt string to print
     * @param args arguments to print
     */
    template<typename... Args>
    static void panic(const std::string &fmt, Args... args) {
        fprintf(stderr, fmt.c_str(), args...);
        fprintf(stderr, "\n");
        exit(1);
    }

    /**
     * @brief Panic the program with a message
     *
     * @param str string to print
     */
    static void panic(const std::string &str) {
        fprintf(stderr, "%s\n", str.c_str());
        exit(1);
    }

    /**
     * @brief Return a formatted string
     *
     * @param fmt string to print
     * @param args arguments to print
     * @return std::string formatted string
     */
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

    /**
     * @brief Return red colored string
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string red(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[31m" + str + "\x1b[0m";
    }

    /**
     * @brief Return red colored string with bold
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string redBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[31;1m" + str + "\x1b[0m";
    }

    /**
     * @brief Return green colored string
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string green(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[32m" + str + "\x1b[0m";
    }

    /**
     * @brief Return green colored string with bold
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string greenBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[32;1m" + str + "\x1b[0m";
    }

    /**
     * @brief Return yellow colored string
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string yellow(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[33m" + str + "\x1b[0m";
    }

    /**
     * @brief Return yellow colored string with bold
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string yellowBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[33;1m" + str + "\x1b[0m";
    }

    /**
     * @brief Return blue colored string
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string blue(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[34m" + str + "\x1b[0m";
    }

    /**
     * @brief Return blue colored string with bold
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string blueBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[34;1m" + str + "\x1b[0m";
    }

    /**
     * @brief Return magenta colored string
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string magenta(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[35m" + str + "\x1b[0m";
    }

    /**
     * @brief Return magenta colored string with bold
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string magentaBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[35;1m" + str + "\x1b[0m";
    }

    /**
     * @brief Return cyan colored string
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string cyan(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[36m" + str + "\x1b[0m";
    }

    /**
     * @brief Return cyan colored string with bold
     *
     * @param str string to color
     * @return std::string colored string
     */
    static std::string cyanBold(const std::string &str) {
        if (!isTTY())
            return str;
        return "\x1b[36;1m" + str + "\x1b[0m";
    }

    /**
     * @brief Give padding to a string
     *
     * @param basicString
     * @param usage
     * @return std::string padded string
     */
    static std::string pad(std::string basicString, ssize_t usage) {
        if (usage < 0)
            return basicString;
        if (basicString.size() >= usage)
            return basicString;
        return basicString + std::string(usage - basicString.size(), ' ');
    }
};


#endif //DAL_FMT_HPP
