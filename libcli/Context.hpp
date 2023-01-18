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

class Context {
private:
    std::map<std::string, Argument *> arguments;
public:
    explicit Context(std::map<std::string, Argument *> arguments);

    std::string *getStringArg(const std::string &key);

    bool *getBoolArg(const std::string &key);
};


#endif //DAL_CONTEXT_HPP
