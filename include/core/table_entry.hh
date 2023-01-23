/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_CORE_TYPE_TABLE_HH
#define DAL_CORE_TYPE_TABLE_HH

#include "ast.hh"
#include "llvm.hh"

namespace dal::core {

enum class type_table_kind {
  type_invalid,
  type_void,
  type_bool,
  type_no_return,
  type_int,
  type_ptr,
  type_array,
};

class type_table_entry {
 public:
  explicit type_table_entry(type_table_kind kind);
  ~type_table_entry() = default;

  void set_name(const std::string& name);
  void set_llvm_type(llvm::Type* type);
  void set_size(unsigned int size);
  void set_align(unsigned int align);
  void set_signed(bool is_signed);

  void set_const_parent_ptr(std::shared_ptr<type_table_entry> ptr);
  void set_mut_parent_ptr(std::shared_ptr<type_table_entry> ptr);

  std::shared_ptr<type_table_entry> get_const_parent_ptr();
  std::shared_ptr<type_table_entry> get_mut_parent_ptr();

  [[nodiscard]] llvm::Type* get_llvm_type() const;
  [[nodiscard]] std::string get_name() const;

 private:
  type_table_kind m_kind;
  std::string m_name;
  llvm::Type* m_llvm_type = nullptr;
  unsigned int m_size = 0;
  unsigned int m_align = 0;
  // integer stuff
  bool m_is_signed = false;

  std::shared_ptr<type_table_entry> m_const_parent_ptr;
  std::shared_ptr<type_table_entry> m_mut_parent_ptr;
};

class import_table {
 public:
  import_table() = default;
  ~import_table() = default;

  void set_source(const std::string& source);
  void set_path(const std::string& path);
  void set_root(std::shared_ptr<ast> root);

  [[nodiscard]] std::string get_path() const;
  std::shared_ptr<ast> get_root();

 private:
  std::string m_source;
  std::string m_path;
  std::shared_ptr<ast> m_root;
};

}  // namespace dal::core

#endif  //DAL_CORE_TYPE_TABLE_HH
