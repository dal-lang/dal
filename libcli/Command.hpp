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

/**
 * @file Command.hpp
 * @brief Command class is the base class for all commands
 * @details This class is the base class for all commands. It is used to
 *         create a command object that can be used to parse the command
 *         line arguments.
 */
class Command {
private:
    std::string name;
    std::string description;
    std::string usage;
    std::map<std::string, Argument *> arguments;
    CommandHandler handler = {};

    /**
     * @brief Fall back when argument doesn't match.
     *
     * @param msg Error message.
     */
    static void fallback(const std::string &msg);

    /**
     * @brief Run the command
     *
     * @param ctx Context of the command
     */
    void run(Context *ctx);

public:
    /**
     * @brief Construct a new Command object
     *
     * @param name Name of the command
     * @param description Description of the command
     * @param usage Usage of the command
     */
    Command(std::string name, std::string description, std::string usage);

    ~Command() = default;

    /**
     * @brief Add an argument to the command
     * @details This function is used to add an argument to the command
     *
     * @param argument Argument to be added
     */
    void addArgument(Argument *argument);

    /**
     * @brief Add a string argument to the command
     * @details This function is used to add a string argument to the command
     *
     * @param arg_name Name of the argument
     * @param arg_desc Description of the argument
     * @param arg_usage Usage of the argument
     * @param is_required Is the argument required
     */
    void addStringArgument(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required = false);

    /**
     * @brief Add a boolean argument to the command
     * @details This function is used to add a boolean argument to the command
     *
     * @param arg_name Name of the argument
     * @param arg_desc Description of the argument
     * @param arg_usage Usage of the argument
     * @param is_required Is the argument required
     */
    void
    addBooleanArgument(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required = false);

    /**
     * @brief Set the handler of the command
     * @details This function is used to set the handler of the command
     *
     * @param command_handler Handler of the command
     */
    void setHandler(CommandHandler command_handler);

    /**
     * @brief Get the Name of the command
     * @details This function is used to get the name of the command
     *
     * @return std::string Name of the command
     */
    std::string getName();

    /**
     * @brief Get the Description of the command
     * @details This function is used to get the description of the command
     *
     * @return std::string Description of the command
     */
    std::string getDescription();

    /**
     * @brief Get the Usage of the command
     * @details This function is used to get the usage of the command
     *
     * @return std::string Usage of the command
     */
    std::string getUsage();

    /**
     * @brief Parse the command line arguments
     *
     * @param args Command line arguments
     * @return Context* Context of the command
     */
    void parse(std::vector<std::string> args);

    /**
     * @brief Print help about this command.
     */
    void printHelp();
};


#endif //DAL_COMMAND_HPP
