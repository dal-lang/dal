/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_COMMAND_HPP
#define DAL_COMMAND_HPP

#include <string>
#include <map>
#include <vector>
#include "Argument.hpp"
#include "Context.hpp"

// Command handler function
typedef void (*CommandHandler)(Context *ctx);

class Command {
private:
    std::string name;
    std::string description;
    std::string usage;
    std::map<std::string, Argument *> arguments;
    CommandHandler handler = {};

    static void fallback(const std::string &msg);

    void run(Context *ctx);

public:
    Command(std::string name, std::string description, std::string usage);

    ~Command() = default;

    void addArgument(Argument *argument);

    void addStringArgument(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required = false);

    void
    addBooleanArgument(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required = false);

    void setHandler(CommandHandler command_handler);

    std::string getName();

    std::string getDescription();

    std::string getUsage();

    void parse(std::vector<std::string> args);

    void printHelp();
};


#endif //DAL_COMMAND_HPP
