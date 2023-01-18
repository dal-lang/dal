/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_ERROR_HPP
#define DAL_ERROR_HPP


#include <string>
#include "Span.hpp"

class Error {
private:
    std::string m_msg;
    Span m_span;

public:
    explicit Error(const std::string &msg, Span span);

    [[noreturn]]
    void panic(const std::string &source);
};


#endif //DAL_ERROR_HPP
