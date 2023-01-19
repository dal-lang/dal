/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "arg.hh"

namespace dal::cli {

    cli_arg::cli_arg(arg_kind kind) {
        this->m_kind = kind;
    }

    void cli_arg::set_name(std::string arg_name) {
        this->m_name = std::move(arg_name);
    }

    void cli_arg::set_desc(std::string arg_desc) {
        this->m_desc = std::move(arg_desc);
    }

    void cli_arg::set_usage(std::string arg_usage) {
        this->m_usage = std::move(arg_usage);
    }

    [[maybe_unused]] void cli_arg::set_kind(arg_kind arg_kind) {
        this->m_kind = arg_kind;
    }

    std::string cli_arg::get_name() {
        return this->m_name;
    }

    [[maybe_unused]] std::string cli_arg::get_desc() {
        return this->m_desc;
    }

    [[maybe_unused]] std::string cli_arg::get_usage() {
        return this->m_usage;
    }

    arg_kind cli_arg::get_kind() {
        return this->m_kind;
    }

    void string_arg::set_value(std::string arg_value) {
        this->m_value = std::move(arg_value);
    }

    [[maybe_unused]] void string_arg::set_default_value(std::string arg_default) {
        this->m_default_value = std::move(arg_default);
    }

    void string_arg::set_required(bool is_required) {
        this->m_required = is_required;
    }

    std::string string_arg::get_value() {
        return this->m_value;
    }

    std::string string_arg::get_default_value() {
        return this->m_default_value;
    }

    bool string_arg::is_required() const {
        return this->m_required;
    }

    void bool_arg::set_value(bool arg_value) {
        this->m_value = arg_value;
    }

    [[maybe_unused]] void bool_arg::set_default_value(bool arg_default) {
        this->m_default_value = arg_default;
    }

    void bool_arg::set_required(bool is_required) {
        this->m_required = is_required;
    }

    bool bool_arg::get_value() const {
        return this->m_value;
    }

    bool bool_arg::get_default_value() const {
        return this->m_default_value;
    }

    bool bool_arg::is_required() const {
        return this->m_required;
    }

} // dal::cli