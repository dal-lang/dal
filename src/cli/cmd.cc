/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */


#include "cmd.hh"
#include "utils/fmt.hh"
#include "utils/utils.hh"

namespace dal::cli {

    cli_cmd::cli_cmd(std::string name, std::string description, std::string usage) {
        this->m_name = std::move(name);
        this->m_desc = std::move(description);
        this->m_usage = std::move(usage);
    }

    void cli_cmd::add_arg(cli_arg *argument) {
        this->m_args.insert(std::pair<std::string, cli_arg *>(argument->get_name(), argument));
    }

    void
    cli_cmd::add_string_arg(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required) {
        auto *argument = new string_arg();
        argument->set_name(std::move(arg_name));
        argument->set_desc(std::move(arg_desc));
        argument->set_usage(std::move(arg_usage));
        argument->set_required(is_required);
        this->add_arg(argument);
    }

    void
    cli_cmd::add_bool_arg(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required) {
        auto *argument = new bool_arg();
        argument->set_name(std::move(arg_name));
        argument->set_desc(std::move(arg_desc));
        argument->set_usage(std::move(arg_usage));
        argument->set_required(is_required);
        this->add_arg(argument);
    }

    void cli_cmd::set_handler(cmd_handler command_handler) {
        this->m_handler = command_handler;
    }

    std::string cli_cmd::get_name() {
        return this->m_name;
    }

    std::string cli_cmd::get_desc() {
        return this->m_desc;
    }

    [[maybe_unused]] std::string cli_cmd::get_usage() {
        return this->m_usage;
    }

    void cli_cmd::parse(std::vector<std::string> args) {
        // Loop through all arguments.
        for (int i = 0; i < args.size(); i++) {
            auto arg = args[i];
            std::string prefix = "--";
            // if argument start with '--', we need to strip it.
            if (utils::hasPrefix(arg, prefix)) {
                arg = arg.substr(prefix.length());
            }
            // Check if the argument exist on the command.
            auto flag = this->m_args.find(arg);
            if (flag->first.empty()) {
                // The argument doesn't match with the available options.
                cli_cmd::fallback(utils::fmt::format("%s: --%s", utils::fmt::red_bold("Unknown options"), arg));
            }

            switch (flag->second->get_kind()) {
                case arg_kind_bool:
                    if (i + 1 >= args.size()) {
                        cli_cmd::fallback(
                                utils::fmt::format("%s: --%s", utils::fmt::red_bold("Missing value for argument"),
                                                   arg));
                    } else if (utils::hasPrefix(args[i + 1], prefix)) {
                        cli_cmd::fallback(
                                utils::fmt::format("%s: --%s", utils::fmt::red_bold("Missing value for argument"),
                                                   arg));
                    }
                    i++;
                    reinterpret_cast<string_arg *>(flag->second)->set_value(args[i]);
                    continue;
                case arg_kind_string:
                    reinterpret_cast<bool_arg *>(flag->second)->set_value(true);
                    continue;
            }
        }

        // Verify if all arguments has been supplied.
        for (const auto &argument: this->m_args) {
            auto arg = argument.second;
            switch (arg->get_kind()) {
                case arg_kind_string: {
                    auto v = reinterpret_cast<bool_arg *>(arg);
                    if (v->is_required() && !v->get_default_value() && !v->get_value())
                        cli_cmd::fallback(
                                utils::fmt::format("%s: --%s", utils::fmt::red_bold("Missing required argument"),
                                                   arg->get_name()));
                    break;
                }
                case arg_kind_bool: {
                    auto v = reinterpret_cast<string_arg *>(arg);
                    if (v->is_required() && v->get_default_value().empty() && v->get_value().empty())
                        cli_cmd::fallback(
                                utils::fmt::format("%s: --%s", utils::fmt::red_bold("Missing required argument"),
                                                   arg->get_name()));
                    break;
                }
            }
        }

        context context(this->m_args);
        this->run(&context);
    }

    void cli_cmd::fallback(const std::string &msg) {
        utils::fmt::panic(msg);
    }

    void cli_cmd::run(context *ctx) {
        if (!this->m_handler) {
            cli_cmd::fallback(
                    utils::fmt::format("%s: %s", utils::fmt::red_bold("Missing handler for command"), this->m_name));
        }
        this->m_handler(ctx);
    }

    void cli_cmd::print_help() {
        utils::fmt::println(utils::fmt::format("%s: %s", utils::fmt::green_bold("Usage"), this->m_usage));
        // If the command has arguments, print it.
        if (!this->m_args.empty()) {
            ssize_t longest_name = 0;
            ssize_t longest_usage = 0;
            // Calculate the longest name and usage to make the output looks good.
            for (const auto &argument: this->m_args) {
                auto arg = argument.second;
                if (arg->get_name().length() > longest_name) {
                    longest_name = arg->get_name().length();
                }
                if (arg->get_usage().length() > longest_usage) {
                    longest_usage = arg->get_usage().length();
                }
            }
            utils::fmt::println("%s:", utils::fmt::green_bold("Arguments"));
            for (const auto &argument: this->m_args) {
                auto arg = argument.second;
                std::string name = arg->get_name();
                std::string usage = arg->get_usage();
                std::string desc = arg->get_desc();
                // Add padding to the name and usage.
                name.append(longest_name - name.length(), ' ');
                usage.append(longest_usage - usage.length(), ' ');
                utils::fmt::println(
                        utils::fmt::format("  %s  %s  %s", utils::fmt::green_bold(name), utils::fmt::yellow(usage),
                                           desc));
            }
        }
    }

} // dal::cli