/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Cursor.hpp"

namespace dal {

    Cursor::Cursor(const std::string &str) {
        this->m_data = std::vector<char>(str.begin(), str.end());
    }

    char Cursor::first() {
        if (this->is_eof()) {
            return 0;
        }

        return this->m_data[0];
    }

    char Cursor::second() {
        if (this->is_eof() || this->m_data.size() < 2) {
            return 0;
        }

        return this->m_data[1];
    }

    bool Cursor::is_eof() {
        return this->m_data.empty();
    }

    char Cursor::bump() {
        if (this->is_eof()) {
            return 0;
        }

        char c = this->m_data[0];
        this->m_data.erase(this->m_data.begin());
        if (c == '\n') {
            this->m_span.inc_end_line();
            this->m_span.set_end_col(0);
        } else {
            this->m_span.inc_end_col();
        }
        this->m_span.inc_end_pos();
        return c;
    }

    void Cursor::eat_while(bool (*pred)(char)) {
        while (!this->is_eof() && pred(this->first())) {
            this->bump();
        }
    }

    Span Cursor::span() {
        return this->m_span;
    }

    Span Cursor::eat_span() {
        auto old_span = this->m_span;
        this->m_span.update();
        return old_span;
    }

} // dal