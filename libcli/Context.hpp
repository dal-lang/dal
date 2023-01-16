/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CONTEXT_HPP
#define DAL_CONTEXT_HPP

#include <string>
#include <map>
#include "Argument.hpp"

/**
 * @file Context.hpp
 * @brief Context class holds the context of the command line that's will be passed to the command handler
 */
class Context {
private:
    std::map<std::string, Argument *> arguments;
public:
    /**
     * @brief Construct new Context.
     *
     * @param arguments List of arguments.
     */
    explicit Context(std::map<std::string, Argument *> arguments);

    /**
     * @brief Get string arguments.
     *
     * @param key The key of arguments.
     * @return std::string Argument value.
     */
    std::string *getStringArg(const std::string &key);

    /**
     * @brief Get bool arguments.
     *
     * @param key The key of arguments.
     * @return bool Argument value.
     */
    bool *getBoolArg(const std::string &key);
};


#endif //DAL_CONTEXT_HPP
