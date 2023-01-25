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
  [[nodiscard]] int len() const;
  [[nodiscard]] int start_pos() const;
  [[nodiscard]] int end_pos() const;
  [[nodiscard]] int start_line() const;
  [[nodiscard]] int end_line() const;
  [[nodiscard]] int start_col() const;
  [[nodiscard]] int end_col() const;

 private:
  int m_start_pos = 0;
  int m_end_pos = 0;
  int m_start_line = 1;
  int m_end_line = 1;
  int m_start_col = 1;
  int m_end_col = 1;
};

}  // namespace dal::core

#endif  //DAL_CORE_SPAN_HH
