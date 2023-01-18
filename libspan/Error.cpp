/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Error.hpp"

Error::Error(const std::string &msg, Span span) {
    this->m_msg = msg;
    this->m_span = span;
}

void Error::panic(const std::string &source) {

}
