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

class CLI {
private:
    std::string app_name;
    std::string app_version;
    std::string app_description;
    std::map<std::string, Command *> commands;
    std::map<std::string, std::string> additional_commands;

    int fallback(const std::string &command = "", const std::string &msg = "", int exit_code = 0);

    int printVersion();

public:
    CLI(std::string app_name, std::string app_version, std::string app_description);

    ~CLI() = default;

    void addCommand(Command *command);

    int parse(int argc, char *argv[]);
};


#endif //DAL_CLI_HPP
