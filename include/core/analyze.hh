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
#include "visitor.hh"

namespace dal::core {

class analyze {
 public:
  explicit analyze(codegen* codegen) : m_codegen(codegen) {}
  ~analyze() = default;

  friend class analyze_fn_decl_visitor;

  void start();

 private:
  codegen* m_codegen;

  void analyze_fn_proto(std::shared_ptr<fn_proto_ast> node, fn_table* entry);
};

class analyze_fn_decl_visitor : public ast_visitor::visitor {
 public:
  explicit analyze_fn_decl_visitor(analyze* analyze,
                                   std::shared_ptr<import_table> import_entry)
      : m_analyze(analyze), m_import_entry(std::move(import_entry)) {}
  ~analyze_fn_decl_visitor() override = default;

  void visit(extern_ast* node) override;
  void visit(fn_def_ast* node) override;

 private:
  analyze* m_analyze;
  std::shared_ptr<import_table> m_import_entry;
};

}  // namespace dal::core

#endif  //DAL_CORE_ANALYZE_HH
