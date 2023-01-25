/*
* Copyright (c) 2023 Ade M Ramdani
*
* This file is part of dal, which is MIT licensed.
* See the file "LICENSE" for more information or visit
* https://opensource.org/licenses/MIT for full license details.
*/

#include <core/analyze.hh>

namespace dal::core {

void analyze::start() {
  {
    for (auto entry : this->m_codegen->m_import_table) {
      if (this->m_codegen->m_verbose) {
        auto table = entry.second;
        ast_visitor::visitor_ptr<root_ast> v;
        table->get_root()->accept(v);
        auto root = v.ptr_;
        for (auto child : root->m_children) {
          if (child->kind() == ast_kind::extern_node ||
              child->kind() == ast_kind::fn_def_node) {
            analyze_fn_decl_visitor an(this, table);
            child->accept(an);
          }
        }
      }
    }
  }
}
void analyze::analyze_fn_proto(std::shared_ptr<fn_proto_ast> node,
                               fn_table* entry) {}

void analyze_fn_decl_visitor::visit(extern_ast* node) {
  auto g = this->m_analyze->m_codegen;
  for (const auto& attr : node->m_attrs) {
    if (attr->m_name == "link") {
      g->m_link_table.insert({attr->m_arg, true});
    } else {
      g->add_error(fmt::format("{}: {}", fmt::red_bold("Unknown attribute"),
                               attr->m_name),
                   attr.get());
    }
  }

  for (const auto& decl : node->m_fns) {
    auto proto = decl->m_proto;

    auto entry = new fn_table();
    entry->set_proto(proto);
    entry->set_is_extern(true);
    entry->set_calling_conv(llvm::CallingConv::C);
    entry->set_import(this->m_import_entry);

    this->m_analyze->analyze_fn_proto(proto, entry);

    g->m_proto_table.push_back(std::shared_ptr<fn_table>(entry));
    this->m_import_entry->add_fn_table(proto->m_name->m_name,
                                       std::shared_ptr<fn_table>(entry));
  }
}

void analyze_fn_decl_visitor::visit(fn_def_ast* node) {}

}  // namespace dal::core