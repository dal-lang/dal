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

/**
 * @brief ArgumentKind is kind of an argument.
 */
enum ArgumentKind {
    ArgumentKindString,
    ArgumentKindBool,
};

/**
 * @file Argument.hpp
 * @brief Argument class is the base class for all arguments
 */
class Argument {
private:
    std::string name;
    std::string description;
    std::string usage;
    ArgumentKind kind;
public:
    explicit Argument(ArgumentKind kind);

    ~Argument() = default;

    /**
     * @brief set the name of the argument
     * @param arg_name Name of the argument
     */
    void setName(std::string arg_name);

    /**
     * @brief set the description of the argument
     * @param arg_desc Description of the argument
     */
    void setDescription(std::string arg_desc);

    /**
     * @brief set the usage of the argument
     * @param arg_usage Usage of the argument
     */
    void setUsage(std::string arg_usage);

    /**
     * @brief set the kind of the argument
     * @param arg_kind kind of the argument
     */
    [[maybe_unused]] void setKind(ArgumentKind arg_kind);

    /**
     * @brief get the name of the argument
     * @return std::string Name of the argument
     */
    std::string getName();

    /**
     * @brief get the description of the argument
     * @return std::string Description of the argument
     */
    [[maybe_unused]] std::string getDescription();

    /**
     * @brief get the usage of the argument
     * @return std::string Usage of the argument
     */
    [[maybe_unused]] std::string getUsage();

    /**
     * @brief get the kind of the argument
     * @return ArgumentKind Type of the argument
     */
    ArgumentKind getKind();
};

/**
 * @brief String argument class
 * @details This class is used to create a string argument
 */
class StringArgument : public Argument {
private:
    std::string value;
    std::string default_value;
    bool required = false;
public:
    StringArgument() : Argument(ArgumentKindString) {}

    ~StringArgument() = default;

    /**
     * @brief set the value of the argument
     * @param arg_value Value of the argument
     */
    void setValue(std::string arg_value);

    /**
     * @brief set the default value of the argument
     * @param arg_default Default value of the argument
     */
    [[maybe_unused]] void setDefaultValue(std::string arg_default);

    /**
     * @brief set the required status of the argument
     * @param is_required Required status of the argument
     */
    void setRequired(bool is_required);

    /**
     * @brief get the value of the argument
     * @return std::string Value of the argument
     */
    std::string getValue();

    /**
     * @brief get the default value of the argument
     * @return std::string Default value of the argument
     */
    std::string getDefaultValue();

    /**
     * @brief check if the argument is required
     * @return bool True if the argument is required, false otherwise
     */
    [[nodiscard]] bool isRequired() const;
};

/**
 * @brief Boolean argument class
 * @details This class is used to create a boolean argument
 */
class BooleanArgument : public Argument {
private:
    bool value = false;
    bool default_value = false;
    bool required = false;
public:
    BooleanArgument() : Argument(ArgumentKindBool) {}

    ~BooleanArgument() = default;

    /**
     * @brief set the value of the argument
     * @param arg_value Value of the argument
     */
    void setValue(bool arg_value);

    /**
     * @brief set the default value of the argument
     * @param arg_default Default value of the argument
     */
    [[maybe_unused]] void setDefaultValue(bool arg_default);

    /**
     * @brief set the required status of the argument
     * @param is_required Required status of the argument
     */
    void setRequired(bool is_required);

    /**
     * @brief get the value of the argument
     * @return bool Value of the argument
     */
    [[nodiscard]] bool getValue() const;

    /**
     * @brief get the default value of the argument
     * @return bool Default value of the argument
     */
    [[nodiscard]] bool getDefaultValue() const;

    /**
     * @brief check if the argument is required
     * @return bool True if the argument is required, false otherwise
     */
    [[nodiscard]] bool isRequired() const;
};


#endif //DAL_ARGUMENT_HPP
