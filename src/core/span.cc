/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/span.hh>

namespace dal::core {

void span::inc_end_pos() {
  this->m_end_pos++;
}

void span::inc_end_line() {
  this->m_end_line++;
}

void span::inc_end_col() {
  this->m_end_col++;
}

void span::set_end_col(int col) {
  this->m_end_col = col;
}

void span::update() {
  this->m_start_pos = this->m_end_pos;
  this->m_start_line = this->m_end_line;
  this->m_start_col = this->m_end_col;
}

int span::len() const {
  return this->m_end_pos - this->m_start_pos;
}

int span::start_pos() const {
  return this->m_start_pos;
}

int span::end_pos() const {
  return this->m_end_pos;
}

int span::start_line() const {
  return this->m_start_line;
}

int span::end_line() const {
  return this->m_end_line;
}

int span::start_col() const {
  return this->m_start_col;
}

int span::end_col() const {
  return this->m_end_col;
}

} // namespace dal::core