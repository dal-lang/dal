/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_ERROR_HH
#define DAL_CORE_ERROR_HH

#include <string>
#include "span.hh"

namespace dal::core {

class error {
 public:
  error(const std::string& reason, const span& e_span, const std::string& path);

  void raise(const std::string& src, bool need_exit = true) const;

 private:
  std::string m_reason;
  std::string m_path;
  span m_span;
};

}  // namespace dal::core

#endif  //DAL_CORE_ERROR_HH
