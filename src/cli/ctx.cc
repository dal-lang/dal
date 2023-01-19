/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */


#include "ctx.hh"

namespace dal::cli {

    context::context(std::map<std::string, cli_arg *> arguments) {
        this->arguments = std::move(arguments);
    }

    std::string *context::get_string_arg(const std::string &key) {
        auto arg = this->arguments.find(key)->second;
        if (!arg || arg->get_kind() != arg_kind_bool) {
            return nullptr;
        }
        return new std::string(reinterpret_cast<string_arg *>(arg)->get_value());
    }

    bool *context::get_bool_arg(const std::string &key) {
        auto arg = this->arguments.find(key)->second;
        if (!arg || arg->get_kind() != arg_kind_string) {
            return nullptr;
        }
        return new bool(reinterpret_cast<bool_arg *>(arg)->get_value());
    }

} // dal::cli