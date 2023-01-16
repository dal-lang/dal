/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */


#include "Context.hpp"

Context::Context(std::map<std::string, Argument *> arguments) {
    this->arguments = std::move(arguments);
}

std::string *Context::getStringArg(const std::string &key) {
    auto arg = this->arguments.find(key)->second;
    if (!arg || arg->getKind() != ArgumentKindString) {
        return nullptr;
    }
    return new std::string(reinterpret_cast<StringArgument *>(arg)->getValue());
}

bool *Context::getBoolArg(const std::string &key) {
    auto arg = this->arguments.find(key)->second;
    if (!arg || arg->getKind() != ArgumentKindBool) {
        return nullptr;
    }
    return new bool(reinterpret_cast<BooleanArgument *>(arg)->getValue());
}
