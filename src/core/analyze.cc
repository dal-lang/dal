/*
* Copyright (c) 2023 Ade M Ramdani
*
* This file is part of dal, which is MIT licensed.
* See the file "LICENSE" for more information or visit
* https://opensource.org/licenses/MIT for full license details.
*/

#include <core/analyze.hh>
#include <core/visitor.hh>
#include <fmt/format.hh>

namespace dal::core {

void analyze::start() {
  for (const auto &import : this->m_g->m_import_table) {
    auto table = import.second;
    ast_visitor::visitor_ptr<root_ast> vis;
    auto root_node = table->get_root();
    root_node->accept(vis);
    auto root = vis.ptr_;
    for (const auto &node : root->m_children) {
      if (node->kind()==ast_kind::extern_node || node->kind()==ast_kind::fn_def_node) {
        this->analyze_fn_decl(node, table);
      }
    }
  }

  for (const auto &import : this->m_g->m_import_table) {
    auto table = import.second;
    ast_visitor::visitor_ptr<root_ast> vis;
    auto root_node = table->get_root();
    root_node->accept(vis);
    auto root = vis.ptr_;
    for (const auto &node : root->m_children) {
      if (node->kind()==ast_kind::fn_def_node) {
        this->analyze_top_level(node, table);
      }
    }
  }
}

void analyze::analyze_fn_decl(const std::shared_ptr<ast> &node, const std::shared_ptr<import_table> &table) {
  if (node->kind()==ast_kind::extern_node) {
    // analyze the attributes.
    ast_visitor::visitor_ptr<extern_ast> vis;
    node->accept(vis);
    auto extern_node = vis.ptr_;
    for (const auto &attr : extern_node->m_attrs) {
      auto name = attr->m_name;
      auto arg = attr->m_arg;
      if (name=="link") {
        this->m_g->m_link_table.insert({arg, true});
      } else {
        this->m_g->add_error(attr.get(), fmt::format("unknown attribute '{}'", name));
      }
    }

    // analyze the function declaration.
    for (const auto &decl : extern_node->m_fns) {
      auto proto = decl->m_proto;

      auto fn_entry = std::make_shared<fn_table>();
      fn_entry->set_proto(proto);
      fn_entry->set_is_external(true);
      fn_entry->set_calling_conv(llvm::CallingConv::C);
      fn_entry->set_import(table);

      this->resolve_fn_proto(proto, fn_entry);
      table->add_fn_table(proto->m_name->m_name, fn_entry);
      if (proto->m_is_pub) {
        auto root = proto->m_owner.lock()->get_root();
        ast_visitor::visitor_ptr<root_ast> v;
        root->accept(v);
        auto mod_name = v.ptr_->m_module_name->m_name;
        this->m_g->m_fn_table.insert({mod_name + "." + proto->m_name->m_name, fn_entry});
        proto->m_cg_node = std::make_shared<codegen_node>();
        proto->m_cg_node->m_fn_proto.table = fn_entry;
      }
    }
  } else {
    ast_visitor::visitor_ptr<fn_def_ast> vis;
    node->accept(vis);
    auto fn_def = vis.ptr_;
    auto proto = fn_def->m_proto;
    auto fn_tab = table->get_fn_table(proto->m_name->m_name);
    bool skip = false;
    if (fn_tab) {
      this->m_g->add_error(fn_def, fmt::format("function '{}' already defined", proto->m_name->m_name));
      node->m_cg_node = std::make_shared<codegen_node>();
      node->m_cg_node->m_fn_def.skip = true;
    } else if (proto->m_is_pub) {
      auto root = proto->m_owner.lock()->get_root();
      ast_visitor::visitor_ptr<root_ast> v;
      root->accept(v);
      auto global_fn = this->m_g->m_fn_table.find(v.ptr_->m_module_name->m_name + "." + proto->m_name->m_name);
      if (global_fn!=this->m_g->m_fn_table.end()) {
        this->m_g->add_error(fn_def, fmt::format("function '{}' already defined", proto->m_name->m_name));
        node->m_cg_node = std::make_shared<codegen_node>();
        node->m_cg_node->m_fn_def.skip = true;
        skip = true;
      }
    }
    if (proto->m_is_variadic) {
      this->m_g->add_error(fn_def, "variadic functions are not supported");
    }
    if (!skip) {
      auto fn_entry = std::make_shared<fn_table>();
      fn_entry->set_proto(proto);
      fn_entry->set_is_external(false);
      fn_entry->set_def(node);
      fn_entry->set_import(table);
      fn_entry->set_calling_conv(llvm::CallingConv::Fast);

      table->add_fn_table(proto->m_name->m_name, fn_entry);
      if (proto->m_is_pub) {
        auto root = proto->m_owner.lock()->get_root();
        ast_visitor::visitor_ptr<root_ast> v;
        root->accept(v);
        auto mod_name = v.ptr_->m_module_name->m_name;
        this->m_g->m_fn_table.insert({mod_name + "." + proto->m_name->m_name, fn_entry});
      }

      this->resolve_fn_proto(proto, fn_entry);

      node->m_cg_node = std::make_shared<codegen_node>();
      node->m_cg_node->m_fn_proto.table = fn_entry;
    }
  }
}

void analyze::resolve_fn_proto(const std::shared_ptr<fn_proto_ast> &node, const std::shared_ptr<fn_table> &table) {
  for (const auto &attr : node->m_attrs) {
    auto name = attr->m_name;
    if (table->get_def()) {
      if (name=="none") {
        table->add_attr(fn_attr_kind::none);
      } else if (name=="always_inline") {
        table->add_attr(fn_attr_kind::always_inline);
      } else {
        this->m_g->add_error(attr.get(), fmt::format("unknown attribute '{}'", name));
      }
    } else {
      this->m_g->add_error(attr.get(), fmt::format("unknown attribute '{}'", name));
    }
  }

  for (const auto &arg : node->m_params) {
    auto type = this->resolve_type(arg->m_type);
    if (type==this->m_g->m_builtin_type.m_no_return) {
      this->m_g->add_error(arg.get(), "no return type is not allowed in function parameters");
    }
  }
  this->resolve_type(node->m_return_type);
}

std::shared_ptr<type_table> analyze::resolve_type(const std::shared_ptr<type_ast> &node) {
  node->m_cg_node = std::make_shared<codegen_node>();
  auto ty_node = node->m_cg_node->m_type;
  switch (node->m_type_kind) {
    case type_kind::primitive: {
      auto name = node->m_primitive->m_name;
      auto tab = this->m_g->m_type_table.find(name);
      if (tab!=this->m_g->m_type_table.end()) {
        ty_node.table = tab->second;
      } else {
        this->m_g->add_error(node.get(), fmt::format("unknown type '{}'", name));
        ty_node.table = this->m_g->m_builtin_type.m_invalid;
      }
      return ty_node.table;
    }
    case type_kind::array: {
      this->resolve_type(node->m_child);
      auto child_ty = node->m_child->m_cg_node->m_type.table;
      if (child_ty==this->m_g->m_builtin_type.m_no_return) {
        this->m_g->add_error(node.get(), "no return type is not allowed in array");
      }
      auto size_node = node->m_size;
      int size = 0;
      if (size_node->m_kind!=ast_kind::int_node) {
        this->m_g->add_error(node.get(), "array size must be an integer");
        size = -1;
      } else {
        auto size_str = size_node->m_value;
        size = std::stoi(size_str);
      }
      ty_node.table = this->get_array_type(child_ty, size);
      return ty_node.table;
    }
    case type_kind::pointer: {
      this->resolve_type(node->m_child);
      auto child_ty = node->m_child->m_cg_node->m_type.table;
      if (child_ty==this->m_g->m_builtin_type.m_no_return) {
        this->m_g->add_error(node.get(), "no return type is not allowed in pointer");
      } else if (child_ty->get_kind()==type_table_kind::type_invalid) {
        return child_ty;
      }
      ty_node.table = this->m_g->get_pointer_to_type(child_ty, !node->m_is_mut);
      return ty_node.table;
    }
  }
}

std::shared_ptr<type_table> analyze::get_array_type(const std::shared_ptr<type_table> &table, int size) {
  auto exist = table->get_array_by_size(size);
  if (exist) {
    return exist;
  } else {
    auto entry = std::make_shared<type_table>(type_table_kind::type_array);
    entry->set_type(llvm::ArrayType::get(table->get_type(), size));
    entry->set_name(fmt::format("[{}; {}]", table->get_name(), size));
    entry->set_size(table->get_size()*size);
    entry->set_align(table->get_align());
    entry->set_array(table);
    this->m_g->m_type_table.insert({entry->get_name(), entry});
    table->add_array_by_size(size, entry);
    return entry;
  }
}

void analyze::analyze_top_level(const std::shared_ptr<ast> &node, const std::shared_ptr<import_table> &table) {
  ast_visitor::visitor_ptr<fn_def_ast> v;
  node->accept(v);
  auto fn_def = v.ptr_;
  if (fn_def->m_cg_node && fn_def->m_cg_node->m_fn_def.skip)
    return;

  auto proto = fn_def->m_proto;
  fn_def->m_cg_node = std::make_shared<codegen_node>();
  auto ctx = new block_ctx(node, nullptr);
  node->m_cg_node->m_fn_def.block = std::shared_ptr<block_ctx>(ctx);
  int index = 0;
  for (const auto &arg : proto->m_params) {
    auto type = arg->m_type->m_cg_node->m_type.table;
    auto var_tab = std::make_shared<local_var_table>();
    var_tab->set_name(arg->m_name->m_name);
    var_tab->set_type(type);
    var_tab->set_is_const(true);
    var_tab->set_decl_node(arg);
    var_tab->set_arg_index(index);
    auto exist = ctx->get_local_var(arg->m_name->m_name);
    if (!exist) {
      ctx->add_local_var(arg->m_name->m_name, var_tab);
    } else {
      this->m_g->add_error(arg.get(), fmt::format("redefinition of '{}'", arg->m_name->m_name));
      if (exist->get_type()==var_tab->get_type()) {
        // do nothing
      } else {
        exist->set_type(this->m_g->m_builtin_type.m_invalid);
      }
    }
  }
}

}  // namespace dal::core