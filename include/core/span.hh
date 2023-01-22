/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_SPAN_HH
#define DAL_CORE_SPAN_HH

namespace dal::core {

class span {
public:
  span() = default;
  ~span() = default;

  void inc_end_pos();
  void inc_end_line();
  void inc_end_col();
  void set_end_col(int col);
  void update();
  int len() const;
  int start_pos() const;
  int end_pos() const;
  int start_line() const;
  int end_line() const;
  int start_col() const;
  int end_col() const;
private:
  int m_start_pos = 0;
  int m_end_pos = 0;
  int m_start_line = 0;
  int m_end_line = 0;
  int m_start_col = 0;
  int m_end_col = 0;
};

} // namespace dal::core

#endif //DAL_CORE_SPAN_HH
