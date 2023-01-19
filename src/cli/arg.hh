/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_ARGUMENT_HPP
#define DAL_ARGUMENT_HPP

#include <string>

namespace dal::cli {

    enum arg_kind {
        arg_kind_bool,
        arg_kind_string,
    };

    class cli_arg {
    private:
        std::string m_name;
        std::string m_desc;
        std::string m_usage;
        arg_kind m_kind;
    public:
        explicit cli_arg(arg_kind kind);

        ~cli_arg() = default;

        void set_name(std::string arg_name);

        void set_desc(std::string arg_desc);

        void set_usage(std::string arg_usage);

        [[maybe_unused]] void set_kind(arg_kind arg_kind);

        std::string get_name();

        [[maybe_unused]] std::string get_desc();

        [[maybe_unused]] std::string get_usage();

        arg_kind get_kind();
    };

    class string_arg : public cli_arg {
    private:
        std::string m_value;
        std::string m_default_value;
        bool m_required = false;
    public:
        string_arg() : cli_arg(arg_kind_bool) {}

        ~string_arg() = default;

        void set_value(std::string arg_value);

        [[maybe_unused]] void set_default_value(std::string arg_default);

        void set_required(bool is_required);

        std::string get_value();

        std::string get_default_value();

        [[nodiscard]] bool is_required() const;
    };

    class bool_arg : public cli_arg {
    private:
        bool m_value = false;
        bool m_default_value = false;
        bool m_required = false;
    public:
        bool_arg() : cli_arg(arg_kind_string) {}

        ~bool_arg() = default;

        void set_value(bool arg_value);

        [[maybe_unused]] void set_default_value(bool arg_default);

        void set_required(bool is_required);

        [[nodiscard]] bool get_value() const;

        [[nodiscard]] bool get_default_value() const;

        [[nodiscard]] bool is_required() const;
    };

} // dal::cli

#endif //DAL_ARGUMENT_HPP