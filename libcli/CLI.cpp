/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "CLI.hpp"
#include <map>
#include <vector>
#include <algorithm>
#include <libutils/Fmt.hpp>

CLI::CLI(std::string app_name, std::string app_version, std::string app_description) {
    this->app_name = std::move(app_name);
    this->app_version = std::move(app_version);
    this->app_description = std::move(app_description);
    this->additional_commands["version, -v, --version"] = "Print the version of the application";
    this->additional_commands["help, -h, --help"] = "Print the help of the application";
}

void CLI::addCommand(Command *command) {
    this->commands.insert(std::pair<std::string, Command *>(command->getName(), command));
}

int CLI::parse(int argc, char **argv) {
    std::string command;

    if (argc == 1) {
        return this->fallback("", "No command given", 1);
    }

    command = argv[1];
    // Check if the command is in the additional commands
    if (command == "version" || command == "-v" || command == "--version") {
        return this->printVersion();
    } else if (command == "help" || command == "-h" || command == "--help") {
        // If after `help` there is a command, print the help of that command
        if (argc > 2) {
            command = argv[2];
            if (this->commands.find(command) != this->commands.end()) {
                this->commands[command]->printHelp();
                return 0;
            } else {
                return this->fallback(command, "Command not found", 1);
            }
        } else {
            return this->fallback();
        }
    }

    // Check if the command is in the commands
    if (this->commands.find(command) != this->commands.end()) {
        // We need to remove the first argument
        std::vector<std::string> args;
        for (int i = 2; i < argc; i++) {
            args.emplace_back(argv[i]);
        }
        auto cmd = this->commands[command];
        cmd->parse(args);
        return 0;
    } else {
        return this->fallback(command, "Unknown command.", 1);
    }
}

int CLI::fallback(const std::string &command, const std::string &msg, int exit_code) {
    if (!command.empty()) {
        Fmt::println<std::string, std::string>("%s `%s`", Fmt::red("Cannot find command"), command);
        return this->fallback("", "", exit_code);
    } else {
        Fmt::println("%s: %s <command> [arguments]", Fmt::greenBold("Usage"), this->app_name);

        // To pretty print the commands, we need to know the longest command name
        // so, we can align the description
        size_t longest_command_name = 0;
        std::vector<std::string> sorted_commands; // Used to sort the commands alphabetically
        for (auto &pair: this->commands) {
            if (pair.first.length() > longest_command_name) {
                longest_command_name = pair.first.length();
            }
            sorted_commands.push_back(pair.first);
        }
        std::sort(sorted_commands.begin(), sorted_commands.end());

        // Print the commands
        Fmt::println("%s:", Fmt::greenBold("Commands"));
        for (auto &item: sorted_commands) {
            Fmt::print("  %s", Fmt::cyanBold(item));
            for (int i = 0; i < longest_command_name - item.length(); i++) {
                Fmt::print(" ");
            }
            Fmt::println("  %s", this->commands[item]->getDescription());
        }
        Fmt::println("");

        // Print the additional commands
        size_t longest_additional_command_name = 0;
        for (auto &pair: this->additional_commands) {
            if (pair.first.length() > longest_additional_command_name) {
                longest_additional_command_name = pair.first.length();
            }
        }
        Fmt::println("%s:", Fmt::greenBold("Additional commands"));
        for (auto &pair: this->additional_commands) {
            Fmt::print("  %s", Fmt::cyanBold(pair.first));
            for (int i = 0; i < longest_additional_command_name - pair.first.length(); i++) {
                Fmt::print(" ");
            }
            Fmt::println("  %s", pair.second);
        }
        Fmt::println("");
    }
    return exit_code;
}

int CLI::printVersion() {
    Fmt::println("%s %s", Fmt::greenBold(this->app_name), this->app_version);
    Fmt::println("%s", this->app_description);
    return 0;
}