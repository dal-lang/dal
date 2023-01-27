/*
* Copyright (c) 2023 Ade M Ramdani
*
* This file is part of dal, which is MIT licensed.
* See the file "LICENSE" for more information or visit
* https://opensource.org/licenses/MIT for full license details.
*/

#include <core/visitor.hh>

namespace dal::core::ast_visitor {

template<>
void visitor_ptr<root_ast>::visit(root_ast *node) {
  this->ptr_ = node;
}

template<>
void visitor_ptr<ident_ast>::visit(ident_ast *node) {
  this->ptr_ = node;
}

template<>
void visitor_ptr<call_ast>::visit(call_ast *node) {
  this->ptr_ = node;
}

template<>
void visitor_ptr<import_ast>::visit(import_ast *node) {
  this->ptr_ = node;
}

template<>
void visitor_ptr<extern_ast>::visit(dal::core::extern_ast *node) {
  this->ptr_ = node;
}

template<>
void visitor_ptr<fn_def_ast>::visit(fn_def_ast *node) {
  this->ptr_ = node;
}

}  // namespace dal::core::ast_visitor