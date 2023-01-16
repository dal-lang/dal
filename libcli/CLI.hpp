/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */


#ifndef DAL_CLI_HPP
#define DAL_CLI_HPP

#include <string>
#include <map>
#include "Command.hpp"

/**
 * @file CLI.hpp
 * @brief CLI class is the command line parser for dal
 */
class CLI {
private:
    std::string app_name;
    std::string app_version;
    std::string app_description;
    std::map<std::string, Command *> commands;
    std::map<std::string, std::string> additional_commands;

    /**
     * @brief Fall back when no command is given or when the command is not found
     * @param command Command that is not found
     * @param msg Message to be printed
     * @param exit_code Exit code to be returned
     */
    int fallback(const std::string &command = "", const std::string &msg = "", int exit_code = 0);

    /**
     * @brief Print the version of the application
     */
    int printVersion();

public:
    /**
     * @brief Construct a new CLI object
     *
     * @param app_name Name of the application
     * @param app_version Version of the application
     * @param app_description Description of the application
     */
    CLI(std::string app_name, std::string app_version, std::string app_description);

    ~CLI() = default;

    /**
     * @brief Add a command to the CLI
     * @details This function is used to add a command to the CLI
     *
     * @param command Command to be added
     */
    void addCommand(Command *command);

    /**
     * @brief Parse the command line arguments
     *
     * @param argc Number of arguments
     * @param argv Arguments
     */
    int parse(int argc, char *argv[]);
};


#endif //DAL_CLI_HPP
