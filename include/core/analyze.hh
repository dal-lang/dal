/*
* Copyright (c) 2023 Ade M Ramdani
*
* This file is part of dal, which is MIT licensed.
* See the file "LICENSE" for more information or visit
* https://opensource.org/licenses/MIT for full license details.
*/

#ifndef DAL_CORE_ANALYZE_HH
#define DAL_CORE_ANALYZE_HH

#include "codegen.hh"

namespace dal::core {

class analyze {
 public:
  explicit analyze(codegen *g) : m_g(g) {}
  ~analyze() = default;

  void start();
 private:
  codegen *m_g;

  // private methods.
  void analyze_fn_decl(const std::shared_ptr<ast> &node, const std::shared_ptr<import_table> &table);
  void analyze_top_level(const std::shared_ptr<ast> &node, const std::shared_ptr<import_table> &table);
  void resolve_fn_proto(const std::shared_ptr<fn_proto_ast> &node, const std::shared_ptr<fn_table> &table);
  std::shared_ptr<type_table> resolve_type(const std::shared_ptr<type_ast> &node);
  std::shared_ptr<type_table> get_array_type(const std::shared_ptr<type_table> &table, int size);
};

}  // namespace dal::core

#endif  //DAL_CORE_ANALYZE_HH
