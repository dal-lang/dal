/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_SPAN_HPP
#define DAL_SPAN_HPP


class Span {
private:
    int m_start_pos;
    int m_end_pos;
    int m_start_line;
    int m_end_line;
    int m_start_col;
    int m_end_col;
public:
    Span() = default;

    void set_start_pos(int start_pos) {
        this->m_start_pos = start_pos;
    }

    void set_end_pos(int end_pos) {
        this->m_end_pos = end_pos;
    }

    void set_start_line(int start_line) {
        this->m_start_line = start_line;
    }

    void set_end_line(int end_line) {
        this->m_end_line = end_line;
    }

    void set_start_col(int start_col) {
        this->m_start_col = start_col;
    }

    void set_end_col(int end_col) {
        this->m_end_col = end_col;
    }

    void inc_start_pos() {
        this->m_start_pos++;
    }

    void inc_end_pos() {
        this->m_end_pos++;
    }

    void inc_start_line() {
        this->m_start_line++;
    }

    void inc_end_line() {
        this->m_end_line++;
    }

    void inc_start_col() {
        this->m_start_col++;
    }

    void inc_end_col() {
        this->m_end_col++;
    }

    void update() {
        this->m_start_pos = this->m_end_pos;
        this->m_start_line = this->m_end_line;
        this->m_start_col = this->m_end_col;
    }

    Span merge(Span &other) const {
        Span span{};
        span.set_start_pos(this->m_start_pos);
        span.set_end_pos(other.m_end_pos);
        span.set_start_line(this->m_start_line);
        span.set_end_line(other.m_end_line);
        span.set_start_col(this->m_start_col);
        span.set_end_col(other.m_end_col);
        return span;
    }

    [[nodiscard]] int start_pos() const {
        return this->m_start_pos;
    }

    [[nodiscard]] int end_pos() const {
        return this->m_end_pos;
    }

    [[nodiscard]] int start_line() const {
        return this->m_start_line;
    }

    [[nodiscard]] int start_col() const {
        return this->m_start_col;
    }

    [[nodiscard]] int len() const {
        return this->m_end_pos - this->m_start_pos;
    }
};


#endif //DAL_SPAN_HPP
