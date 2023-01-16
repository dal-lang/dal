/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */


#include "Command.hpp"
#include <libutils/Utils.hpp>
#include <libutils/Fmt.hpp>

Command::Command(std::string name, std::string description, std::string usage) {
    this->name = std::move(name);
    this->description = std::move(description);
    this->usage = std::move(usage);
}

void Command::addArgument(Argument *argument) {
    this->arguments.insert(std::pair<std::string, Argument *>(argument->getName(), argument));
}

void Command::addStringArgument(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required) {
    auto *argument = new StringArgument();
    argument->setName(std::move(arg_name));
    argument->setDescription(std::move(arg_desc));
    argument->setUsage(std::move(arg_usage));
    argument->setRequired(is_required);
    this->addArgument(argument);
}

void Command::addBooleanArgument(std::string arg_name, std::string arg_desc, std::string arg_usage, bool is_required) {
    auto *argument = new BooleanArgument();
    argument->setName(std::move(arg_name));
    argument->setDescription(std::move(arg_desc));
    argument->setUsage(std::move(arg_usage));
    argument->setRequired(is_required);
    this->addArgument(argument);
}

void Command::setHandler(CommandHandler command_handler) {
    this->handler = command_handler;
}

std::string Command::getName() {
    return this->name;
}

std::string Command::getDescription() {
    return this->description;
}

[[maybe_unused]] std::string Command::getUsage() {
    return this->usage;
}

void Command::parse(std::vector<std::string> args) {
    // Loop through all arguments.
    for (int i = 0; i < args.size(); i++) {
        auto arg = args[i];
        std::string prefix = "--";
        // if argument start with '--', we need to strip it.
        if (Utils::hasPrefix(arg, prefix)) {
            arg = arg.substr(prefix.length());
        }
        // Check if the argument exist on the command.
        auto flag = this->arguments.find(arg);
        if (flag->first.empty()) {
            // The argument doesn't match with the available options.
            Command::fallback(Fmt::format("%s: --%s", Fmt::redBold("Unknown options"), arg));
        }

        switch (flag->second->getKind()) {
            case ArgumentKindString:
                if (i + 1 >= args.size()) {
                    Command::fallback(Fmt::format("%s: --%s", Fmt::redBold("Missing value for argument"), arg));
                } else if (Utils::hasPrefix(args[i + 1], prefix)) {
                    Command::fallback(Fmt::format("%s: --%s", Fmt::redBold("Missing value for argument"), arg));
                }
                i++;
                reinterpret_cast<StringArgument *>(flag->second)->setValue(args[i]);
                continue;
            case ArgumentKindBool:
                reinterpret_cast<BooleanArgument *>(flag->second)->setValue(true);
                continue;
        }
    }

    // Verify if all arguments has been supplied.
    for (const auto &argument: this->arguments) {
        auto arg = argument.second;
        switch (arg->getKind()) {
            case ArgumentKindBool: {
                auto v = reinterpret_cast<BooleanArgument *>(arg);
                if (v->isRequired() && !v->getDefaultValue() && !v->getValue())
                    Command::fallback(
                            Fmt::format("%s: --%s", Fmt::redBold("Missing required argument"), arg->getName()));
                break;
            }
            case ArgumentKindString: {
                auto v = reinterpret_cast<StringArgument *>(arg);
                if (v->isRequired() && v->getDefaultValue().empty() && v->getValue().empty())
                    Command::fallback(
                            Fmt::format("%s: --%s", Fmt::redBold("Missing required argument"), arg->getName()));
                break;
            }
        }
    }

    Context context(this->arguments);
    this->run(&context);
}

void Command::fallback(const std::string &msg) {
    Fmt::panic(msg);
}

void Command::run(Context *ctx) {
    if (!this->handler) {
        Command::fallback(Fmt::format("%s: %s", Fmt::redBold("Missing handler for command"), this->name));
    }
    this->handler(ctx);
}

void Command::printHelp() {
    Fmt::println(Fmt::format("%s: %s", Fmt::greenBold("Usage"), this->usage));
    // If the command has arguments, print it.
    if (!this->arguments.empty()) {
        ssize_t longest_name = 0;
        ssize_t longest_usage = 0;
        // Calculate the longest name and usage to make the output looks good.
        for (const auto &argument: this->arguments) {
            auto arg = argument.second;
            if (arg->getName().length() > longest_name) {
                longest_name = arg->getName().length();
            }
            if (arg->getUsage().length() > longest_usage) {
                longest_usage = arg->getUsage().length();
            }
        }
        Fmt::println(Fmt::format("%s:", Fmt::greenBold("Arguments")));
        for (const auto &argument: this->arguments) {
            auto arg = argument.second;
            std::string name = arg->getName();
            std::string usage = arg->getUsage();
            std::string desc = arg->getDescription();
            // Add padding to the name and usage.
            name.append(longest_name - name.length(), ' ');
            usage.append(longest_usage - usage.length(), ' ');
            Fmt::println(Fmt::format("  %s  %s  %s", Fmt::greenBold(name), Fmt::yellow(usage), desc));
        }
    }
}