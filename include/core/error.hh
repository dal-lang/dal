/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_ERROR_HH
#define DAL_CORE_ERROR_HH

#include "span.hh"
#include <string>

namespace dal::core {

class error {
public:
  error(const std::string &reason, const span &e_span);

  [[noreturn]]
  void raise(const std::string &src) const;
private:
  std::string m_reason;
  span m_span;
};

} // namespace dal::core

#endif //DAL_CORE_ERROR_HH
