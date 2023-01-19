/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "cli.hh"
#include <map>
#include <vector>
#include <algorithm>
#include "utils/fmt.hh"

using namespace dal::utils;

namespace dal::cli {

    cli_app::cli_app(const std::string &app_name, const std::string &app_version, const std::string &app_description) {
        this->m_name = app_name;
        this->m_ver = app_version;
        this->m_desc = app_description;
        this->m_additional["version, -v, --version"] = "Print the version of the application";
        this->m_additional["help, -h, --help"] = "Print the help of the application";
    }

    void cli_app::add_command(cli_cmd *command) {
        this->m_commands.insert(std::pair<std::string, cli_cmd *>(command->get_name(), command));
    }

    int cli_app::parse(int argc, char **argv) {
        std::string command;

        if (argc == 1) {
            return this->fallback("", "No command given", 1);
        }

        command = argv[1];
        // Check if the command is in the additional commands
        if (command == "version" || command == "-v" || command == "--version") {
            return this->print_version();
        } else if (command == "help" || command == "-h" || command == "--help") {
            // If after `help` there is a command, print the help of that command
            if (argc > 2) {
                command = argv[2];
                if (this->m_commands.find(command) != this->m_commands.end()) {
                    this->m_commands[command]->print_help();
                    return 0;
                } else {
                    return this->fallback(command, "Command not found", 1);
                }
            } else {
                return this->fallback();
            }
        }

        // Check if the command is in the commands
        if (this->m_commands.find(command) != this->m_commands.end()) {
            // We need to remove the first argument
            std::vector<std::string> args;
            for (int i = 2; i < argc; i++) {
                args.emplace_back(argv[i]);
            }
            auto cmd = this->m_commands[command];
            cmd->parse(args);
            return 0;
        } else {
            return this->fallback(command, "Unknown command.", 1);
        }
    }

    int cli_app::fallback(const std::string &command, const std::string &msg, int exit_code) {
        if (!msg.empty())
            fmt::println("{}: {}", fmt::red_bold("error"), msg);
        if (!command.empty()) {
            if (this->m_commands.find(command) != this->m_commands.end()) {
                this->m_commands[command]->print_help();
                return exit_code;
            } else {
                fmt::print("{}:", fmt::red_bold("error"));
                fmt::println(" command '{}' not found", command);
            }
        }
        fmt::println("{}: {} <commands> [options]", fmt::green_bold("Usage"), this->m_name);
        if (!this->m_desc.empty())
            fmt::println("{}: {}", fmt::green_bold("Description"), this->m_desc);

        if (!this->m_commands.empty()) {
            fmt::println("{}:", fmt::green_bold("Commands"));
            for (auto &cmd: this->m_commands) {
                fmt::println("  {} - {}", fmt::green_bold(cmd.second->get_name()), cmd.second->get_desc());
            }
        }
        return 0;
    }

    int cli_app::print_version() {
        fmt::println("{} {}", fmt::green_bold(this->m_name), this->m_ver);
        fmt::println("{}", this->m_desc);
        return 0;
    }

} // dal