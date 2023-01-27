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

type_table::type_table(type_table_kind kind) {
  this->m_kind = kind;
}

void type_table::set_name(const std::string &name) {
  this->m_name = name;
}

void type_table::set_type(llvm::Type *type) {
  this->m_type = type;
}

void type_table::set_size(unsigned int size) {
  this->m_size = size;
}

void type_table::set_align(unsigned int align) {
  this->m_align = align;
}

void type_table::set_is_signed(bool is_signed) {
  this->m_is_signed = is_signed;
}

void type_table::set_pointee(std::shared_ptr<type_table> pointee) {
  this->m_pointee = std::move(pointee);
}

void type_table::set_pointee_is_const(bool pointee_is_const) {
  this->m_pointee_is_const = pointee_is_const;
}

void type_table::set_const_pointee(std::shared_ptr<type_table> const_pointee) {
  this->m_const_pointee = std::move(const_pointee);
}

void type_table::set_mut_pointee(std::shared_ptr<type_table> mut_pointee) {
  this->m_mut_pointee = std::move(mut_pointee);
}

void type_table::set_array(std::shared_ptr<type_table> array) {
  this->m_array = std::move(array);
}

void type_table::add_array_by_size(int size, std::shared_ptr<type_table> array) {
  this->m_array_by_size.insert({size, std::move(array)});
}

type_table_kind type_table::get_kind() const {
  return this->m_kind;
}

std::string type_table::get_name() const {
  return this->m_name;
}

llvm::Type *type_table::get_type() {
  return this->m_type;
}

unsigned int type_table::get_size() const {
  return this->m_size;
}

unsigned int type_table::get_align() const {
  return this->m_align;
}

bool type_table::get_is_signed() const {
  return this->m_is_signed;
}

std::shared_ptr<type_table> type_table::get_pointee() {
  return this->m_pointee;
}

bool type_table::get_pointee_is_const() const {
  return this->m_pointee_is_const;
}

std::shared_ptr<type_table> type_table::get_const_pointee() {
  return this->m_const_pointee;
}

std::shared_ptr<type_table> type_table::get_mut_pointee() {
  return this->m_mut_pointee;
}

std::shared_ptr<type_table> type_table::get_array() {
  return this->m_array;
}

std::shared_ptr<type_table> type_table::get_array_by_size(int size) {
  auto it = this->m_array_by_size.find(size);
  if (it!=this->m_array_by_size.end()) {
    return it->second;
  }
  return nullptr;
}

void import_table::set_source(const std::string &source) {
  this->m_source = source;
}

void import_table::set_path(const std::string &path) {
  this->m_path = path;
}

void import_table::set_root(std::shared_ptr<ast> root) {
  this->m_root = std::move(root);
}

void import_table::add_fn_table(const std::string &name,
                                std::shared_ptr<fn_table> fn_table) {
  this->m_fn_table[name] = std::move(fn_table);
}

std::string import_table::get_source() const {
  return this->m_source;
}

std::string import_table::get_path() const {
  return this->m_path;
}

std::shared_ptr<ast> import_table::get_root() const {
  return this->m_root;
}

std::shared_ptr<fn_table> import_table::get_fn_table(
    const std::string &name) const {
  auto it = this->m_fn_table.find(name);
  if (it!=this->m_fn_table.end()) {
    return it->second;
  }
  return nullptr;
}

void fn_table::set_proto(std::shared_ptr<fn_proto_ast> proto) {
  this->m_proto = std::move(proto);
}

void fn_table::set_def(std::shared_ptr<ast> def) {
  this->m_def = std::move(def);
}

void fn_table::set_import(std::shared_ptr<import_table> import) {
  this->m_import = std::move(import);
}

void fn_table::add_attr(fn_attr_kind attr) {
  this->m_attrs.push_back(attr);
}

void fn_table::set_calling_conv(llvm::CallingConv::ID calling_conv) {
  this->m_calling_conv = calling_conv;
}

void fn_table::set_is_external(bool is_external) {
  this->m_is_external = is_external;
}

std::shared_ptr<fn_proto_ast> fn_table::get_proto() {
  return this->m_proto;
}

std::shared_ptr<ast> fn_table::get_def() {
  return this->m_def;
}

std::shared_ptr<import_table> fn_table::get_import() {
  return this->m_import;
}

std::vector<fn_attr_kind> fn_table::get_attrs() const {
  return this->m_attrs;
}

llvm::CallingConv::ID fn_table::get_calling_conv() const {
  return this->m_calling_conv;
}

bool fn_table::get_is_external() const {
  return this->m_is_external;
}

void local_var_table::set_name(const std::string &name) {
  local_var_table::m_name = name;
}

void local_var_table::set_type(const std::shared_ptr<type_table> &type) {
  local_var_table::m_type = type;
}

void local_var_table::set_value(llvm::Value *value) {
  local_var_table::m_value = value;
}

void local_var_table::set_is_const(bool is_const) {
  local_var_table::m_is_const = is_const;
}

void local_var_table::set_is_ptr(bool is_ptr) {
  local_var_table::m_is_ptr = is_ptr;
}

void local_var_table::set_decl_node(const std::shared_ptr<ast> &decl_node) {
  local_var_table::m_decl_node = decl_node;
}

void local_var_table::set_arg_index(int index) {
  local_var_table::m_arg_index = index;
}

std::string local_var_table::get_name() const {
  return m_name;
}

std::shared_ptr<type_table> local_var_table::get_type() {
  return m_type;
}

llvm::Value *local_var_table::get_value() {
  return m_value;
}

bool local_var_table::get_is_const() const {
  return m_is_const;
}

bool local_var_table::get_is_ptr() const {
  return m_is_ptr;
}

std::shared_ptr<ast> local_var_table::get_decl_node() {
  return m_decl_node;
}

int local_var_table::get_arg_index() const {
  return m_arg_index;
}

}  // namespace dal::core