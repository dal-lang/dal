/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CURSOR_HPP
#define DAL_CURSOR_HPP

#include <vector>
#include <string>
#include "Token.hpp"

namespace dal {

    class Cursor {
    private:
        std::vector<char> m_data;
        Span m_span{};

    public:
        explicit Cursor(const std::string &str);

        char first();

        char second();

        bool is_eof();

        char bump();

        void eat_while(bool (*pred)(char));

        Span span();

        Span eat_span();
    };

} // dal

#endif //DAL_CURSOR_HPP
