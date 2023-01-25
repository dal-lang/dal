/*
* Copyright (c) 2023 Ade M Ramdani
*
* This file is part of dal, which is MIT licensed.
* See the file "LICENSE" for more information or visit
* https://opensource.org/licenses/MIT for full license details.
*/

#ifndef DAL_CORE_VISITOR_HH
#define DAL_CORE_VISITOR_HH

#include "ast.hh"

namespace dal::core::ast_visitor {

class visitor {
 public:
  visitor() = default;
  virtual ~visitor() = default;

  virtual void visit(root_ast* node) {}
  virtual void visit(import_ast* node) {}
  virtual void visit(fn_proto_ast* node) {}
  virtual void visit(fn_decl_ast* node) {}
  virtual void visit(fn_def_ast* node) {}
  virtual void visit(fn_param_ast* node) {}
  virtual void visit(type_ast* node) {}
  virtual void visit(block_ast* node) {}
  virtual void visit(extern_ast* node) {}
  virtual void visit(attr_ast* node) {}
  virtual void visit(return_ast* node) {}
  virtual void visit(var_decl_ast* node) {}
  virtual void visit(bin_op_ast* node) {}
  virtual void visit(un_op_ast* node) {}
  virtual void visit(cast_ast* node) {}
  virtual void visit(no_ret_ast* node) {}
  virtual void visit(ident_ast* node) {}
  virtual void visit(call_ast* node) {}
  virtual void visit(array_index_ast* node) {}
  virtual void visit(void_ast* node) {}
  virtual void visit(if_ast* node) {}
  virtual void visit(string_ast* node) {}
  virtual void visit(int_ast* node) {}
  virtual void visit(bool_ast* node) {}
};

template <typename T>
class visitor_ptr : public visitor {
 public:
  visitor_ptr() = default;
  ~visitor_ptr() override = default;

  T* ptr_;

  void visit(T* node) override;
};

}  // namespace dal::core::ast_visitor

#endif  //DAL_CORE_VISITOR_HH
