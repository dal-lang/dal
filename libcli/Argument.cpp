/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Argument.hpp"

Argument::Argument(ArgumentKind kind) {
    this->kind = kind;
}

void Argument::setName(std::string arg_name) {
    this->name = std::move(arg_name);
}

void Argument::setDescription(std::string arg_desc) {
    this->description = std::move(arg_desc);
}

void Argument::setUsage(std::string arg_usage) {
    this->usage = std::move(arg_usage);
}

[[maybe_unused]] void Argument::setKind(ArgumentKind arg_kind) {
    this->kind = arg_kind;
}

std::string Argument::getName() {
    return this->name;
}

[[maybe_unused]] std::string Argument::getDescription() {
    return this->description;
}

[[maybe_unused]] std::string Argument::getUsage() {
    return this->usage;
}

ArgumentKind Argument::getKind() {
    return this->kind;
}

void StringArgument::setValue(std::string arg_value) {
    this->value = std::move(arg_value);
}

[[maybe_unused]] void StringArgument::setDefaultValue(std::string arg_default) {
    this->default_value = std::move(arg_default);
}

void StringArgument::setRequired(bool is_required) {
    this->required = is_required;
}

std::string StringArgument::getValue() {
    return this->value;
}

std::string StringArgument::getDefaultValue() {
    return this->default_value;
}

bool StringArgument::isRequired() const {
    return this->required;
}

void BooleanArgument::setValue(bool arg_value) {
    this->value = arg_value;
}

[[maybe_unused]] void BooleanArgument::setDefaultValue(bool arg_default) {
    this->default_value = arg_default;
}

void BooleanArgument::setRequired(bool is_required) {
    this->required = is_required;
}

bool BooleanArgument::getValue() const {
    return this->value;
}

bool BooleanArgument::getDefaultValue() const {
    return this->default_value;
}

bool BooleanArgument::isRequired() const {
    return this->required;
}