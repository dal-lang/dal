/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/table_entry.hh>
#include <utility>

namespace dal::core {

type_table_entry::type_table_entry(type_table_kind kind) {
  this->m_kind = kind;
}

void type_table_entry::set_name(const std::string& name) {
  this->m_name = name;
}

void type_table_entry::set_llvm_type(llvm::Type* type) {
  this->m_llvm_type = type;
}

void type_table_entry::set_size(unsigned int size) {
  this->m_size = size;
}

void type_table_entry::set_align(unsigned int align) {
  this->m_align = align;
}
std::shared_ptr<type_table_entry> type_table_entry::get_const_parent_ptr() {
  return this->m_const_parent_ptr;
}
std::shared_ptr<type_table_entry> type_table_entry::get_mut_parent_ptr() {
  return this->m_mut_parent_ptr;
}
void type_table_entry::set_const_parent_ptr(
    std::shared_ptr<type_table_entry> ptr) {
  this->m_const_parent_ptr = std::move(ptr);
}
void type_table_entry::set_mut_parent_ptr(
    std::shared_ptr<type_table_entry> ptr) {
  this->m_mut_parent_ptr = std::move(ptr);
}
llvm::Type* type_table_entry::get_llvm_type() const {
  return this->m_llvm_type;
}
std::string type_table_entry::get_name() const {
  return this->m_name;
}

void type_table_entry::set_signed(bool is_signed) {
  this->m_is_signed = is_signed;
}

void import_table::set_source(const std::string& source) {
  this->m_source = source;
}

void import_table::set_path(const std::string& path) {
  this->m_path = path;
}

void import_table::set_root(std::shared_ptr<ast> root) {
  this->m_root = std::move(root);
}

void import_table::add_fn_table(const std::string& name,
                                std::shared_ptr<fn_table> table) {
  this->m_fn_table[name] = std::move(table);
}

std::string import_table::get_path() const {
  return this->m_path;
}

std::string import_table::get_source() const {
  return this->m_source;
}

std::shared_ptr<ast> import_table::get_root() {
  return this->m_root;
}

void fn_table::set_proto(std::shared_ptr<fn_proto_ast> proto) {
  this->m_proto = std::move(proto);
}

void fn_table::set_def(std::shared_ptr<fn_def_ast> def) {
  this->m_def = std::move(def);
}

void fn_table::set_import(const std::shared_ptr<import_table>& import) {
  this->m_import = import;
}

void fn_table::add_attr(fn_attr_kind attr) {
  this->m_attrs.push_back(attr);
}

void fn_table::set_calling_conv(llvm::CallingConv::ID calling_conv) {
  this->m_calling_conv = calling_conv;
}

void fn_table::set_is_extern(bool is_extern) {
  this->m_is_extern = is_extern;
}

}  // namespace dal::core