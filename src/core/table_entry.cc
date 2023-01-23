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
std::string import_table::get_path() const {
  return this->m_path;
}

std::shared_ptr<ast> import_table::get_root() {
  return this->m_root;
}

}  // namespace dal::core