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

enum ArgumentKind {
    ArgumentKindString,
    ArgumentKindBool,
};

class Argument {
private:
    std::string name;
    std::string description;
    std::string usage;
    ArgumentKind kind;
public:
    explicit Argument(ArgumentKind kind);

    ~Argument() = default;

    void setName(std::string arg_name);

    void setDescription(std::string arg_desc);

    void setUsage(std::string arg_usage);

    [[maybe_unused]] void setKind(ArgumentKind arg_kind);

    std::string getName();

    [[maybe_unused]] std::string getDescription();

    [[maybe_unused]] std::string getUsage();

    ArgumentKind getKind();
};

class StringArgument : public Argument {
private:
    std::string value;
    std::string default_value;
    bool required = false;
public:
    StringArgument() : Argument(ArgumentKindString) {}

    ~StringArgument() = default;

    void setValue(std::string arg_value);

    [[maybe_unused]] void setDefaultValue(std::string arg_default);

    void setRequired(bool is_required);

    std::string getValue();

    std::string getDefaultValue();

    [[nodiscard]] bool isRequired() const;
};

class BooleanArgument : public Argument {
private:
    bool value = false;
    bool default_value = false;
    bool required = false;
public:
    BooleanArgument() : Argument(ArgumentKindBool) {}

    ~BooleanArgument() = default;

    void setValue(bool arg_value);

    [[maybe_unused]] void setDefaultValue(bool arg_default);

    void setRequired(bool is_required);

    [[nodiscard]] bool getValue() const;

    [[nodiscard]] bool getDefaultValue() const;

    [[nodiscard]] bool isRequired() const;
};


#endif //DAL_ARGUMENT_HPP
