/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <config.h>
#include <core/codegen.hh>
#include <core/os.hh>
#include <core/tokenizer.hh>
#include <core/parser.hh>
#include <fmt/core.hh>
#include <core/visitor.hh>
#include <core/analyze.hh>

namespace dal::core {

void codegen::set_root_dir(const std::string &root_dir) {
  this->m_root_dir = root_dir;
}

void codegen::set_build_type(codegen_build_type build_type) {
  this->m_build_type = build_type;
}

void codegen::set_out_type(codegen_out_type out_type) {
  this->m_out_type = out_type;
}

void codegen::set_out_path(const std::string &out_path) {
  this->m_out_path = out_path;
}

void codegen::set_is_statically_linked(bool is_statically_linked) {
  this->m_is_statically_linked = is_statically_linked;
}

void codegen::set_is_verbose(bool is_verbose) {
  this->m_is_verbose = is_verbose;
}

void codegen::set_is_strip_symbols(bool is_strip_symbols) {
  this->m_is_strip_symbols = is_strip_symbols;
}

void codegen::generate(const std::string &file_path) {
  this->prepare(file_path);

  this->m_root_ast = this->add_code(file_path);

  if (this->m_is_verbose) {
    fmt::println("\nSemantic analysis");
    fmt::println("----------------------------------------");
  }

  analyze analyzer(this);
  analyzer.start();

  if (!this->m_errors.empty()) {
    for (const auto &err : this->m_errors) {
      err.raise(false);
    }
    std::exit(1);
  }

  if (this->m_is_verbose) {
    fmt::println("All good!");
    fmt::println("\nCode generation");
    fmt::println("----------------------------------------");
  }
}

void codegen::prepare(const std::string &file_path) {
  this->m_dal_search_paths.push_back(this->m_root_dir);
  this->m_dal_search_paths.emplace_back(DAL_STD_DIR);

  // Split the file path into directory and file name.
  std::string file_dir;
  std::string file_name;
  std::error_code ec;
  os::split_path(file_path, file_dir, file_name, ec);
  if (ec) {
    fmt::panic("{}: {}", fmt::red_bold("error"), ec.message());
  }
  // Initialize llvm.
  llvm_init();

  std::string triple = llvm::sys::getDefaultTargetTriple();
  std::string error;
  auto target = llvm::TargetRegistry::lookupTarget(triple, error);
  if (!target) {
    fmt::panic("{}", error);
  }
  auto cpu = llvm::sys::getHostCPUName().str();
  auto features = llvm_get_host_cpu_features();
  auto opt_level = this->m_build_type==codegen_build_type::debug
                   ? llvm::CodeGenOpt::None
                   : llvm::CodeGenOpt::Aggressive;
  auto reloc_model =
      this->m_is_statically_linked ? llvm::Reloc::Static : llvm::Reloc::PIC_;
  auto target_machine = llvm_create_target_machine(
      *target, triple, cpu, features, reloc_model, opt_level);
  auto data_layout = target_machine->createDataLayout();
  auto context = new llvm::LLVMContext();
  auto module = new llvm::Module(file_name, *context);
  module->setDataLayout(data_layout);
  module->setTargetTriple(triple);
  auto builder = new llvm::IRBuilder<>(*context);

  this->m_context = std::shared_ptr<llvm::LLVMContext>(context);
  this->m_module = std::shared_ptr<llvm::Module>(module);
  this->m_builder = std::shared_ptr<llvm::IRBuilder<>>(builder);
  this->m_target_machine = std::shared_ptr<llvm::TargetMachine>(target_machine);

  this->prepare_builtin_type();
}

void codegen::prepare_builtin_type() {
  {
    auto type = std::make_shared<type_table>(type_table_kind::type_invalid);
    type->set_name("invalid");
    this->m_builtin_type.m_invalid = type;
  }
  {
    auto type = std::make_shared<type_table>(type_table_kind::type_bool);
    type->set_name("bool");
    type->set_type(llvm::Type::getInt1Ty(*this->m_context));
    type->set_size(1);
    type->set_align(8);
    this->m_builtin_type.m_bool = type;
    this->m_type_table.insert({type->get_name(), type});
  }
  {
    auto type = std::make_shared<type_table>(type_table_kind::type_int);
    type->set_name("u8");
    type->set_type(llvm::Type::getInt8Ty(*this->m_context));
    type->set_size(1);
    type->set_align(8);
    this->m_builtin_type.m_u8 = type;
    this->m_type_table.insert({type->get_name(), type});
  }
  {
    this->m_builtin_type.m_str_literal =
        this->get_pointer_to_type(this->m_builtin_type.m_u8, true);
  }
  {
    auto type = std::make_shared<type_table>(type_table_kind::type_int);
    type->set_name("i32");
    type->set_type(llvm::Type::getInt32Ty(*this->m_context));
    type->set_size(32);
    type->set_align(32);
    type->set_is_signed(true);
    this->m_builtin_type.m_i32 = type;
    this->m_type_table.insert({type->get_name(), type});
  }
  {
    auto type = std::make_shared<type_table>(type_table_kind::type_int);
    type->set_name("isize");
    type->set_type(
        llvm::Type::getIntNTy(*this->m_context, this->m_ptr_size*8));
    type->set_size(this->m_ptr_size*8);
    type->set_align(this->m_ptr_size*8);
    type->set_is_signed(true);
    this->m_builtin_type.m_isize = type;
    this->m_type_table.insert({type->get_name(), type});
  }
  {
    auto type = std::make_shared<type_table>(type_table_kind::type_void);
    type->set_name("void");
    type->set_type(llvm::Type::getVoidTy(*this->m_context));
    this->m_builtin_type.m_void = type;
    this->m_type_table.insert({type->get_name(), type});
  }
  {
    auto type = std::make_shared<type_table>(type_table_kind::type_no_return);
    type->set_name("!");
    type->set_type(llvm::Type::getVoidTy(*this->m_context));
    this->m_builtin_type.m_no_return = type;
    this->m_type_table.insert({type->get_name(), type});
  }
}

std::shared_ptr<type_table> codegen::get_pointer_to_type(
    const std::shared_ptr<type_table> &type, bool is_const) {
  auto parent_ptr =
      is_const ? type->get_const_pointee() : type->get_mut_pointee();
  if (parent_ptr) {
    return parent_ptr;
  } else {
    auto new_type = std::make_shared<type_table>(type_table_kind::type_ptr);
    new_type->set_name(
        fmt::format("*{} {}", is_const ? "const" : "mut", type->get_name()));
    new_type->set_type(llvm::PointerType::get(type->get_type(), 0));
    new_type->set_size(this->m_ptr_size*8);
    new_type->set_align(this->m_ptr_size*8);
    if (is_const) {
      type->set_const_pointee(new_type);
    } else {
      type->set_mut_pointee(new_type);
    }
    this->m_type_table.insert({new_type->get_name(), new_type});
    return new_type;
  }
}

std::shared_ptr<ast> codegen::add_code(const std::string &file_path) {
  if (this->m_is_verbose) {
    fmt::println("Parsing file: {}", file_path);
    fmt::println("----------------------------------------");
  }

  std::error_code ec;
  auto content = os::read_file(file_path, ec);
  if (ec) {
    fmt::panic("Failed to read file: {}", file_path);
  }

  if (this->m_is_verbose) {
    fmt::println("{}\n", content);
    fmt::println("Tokenizing file: {}", file_path);
    fmt::println("----------------------------------------");
  }

  tokenizer tk;
  auto tokens = tk.tokenize(content, file_path);
  if (tk.has_error())
    tk.print_error();

  if (this->m_is_verbose) {
    for (auto &token : tokens) {
      fmt::println("{}", token.t_str(content));
    }
    fmt::println("Parsing file: {}", file_path);
    fmt::println("----------------------------------------");
  }

  auto import = std::make_shared<import_table>();
  import->set_path(file_path);
  import->set_source(content);
  parser ps(content, tokens, import);
  auto parsed = ps.parse();
  auto res_ast = std::shared_ptr<ast>(parsed);

  if (this->m_is_verbose) {
    fmt::println("{}\n", res_ast->to_string(0));
  }

  import->set_root(res_ast);
  this->m_import_table.insert({file_path, import});

  ast_visitor::visitor_ptr<root_ast> vis;
  res_ast->accept(vis);

  for (const auto &child : vis.ptr_->m_children) {
    if (child->kind()!=ast_kind::import_node) {
      continue;
    }

    ast_visitor::visitor_ptr<import_ast> import_vis;
    child->accept(import_vis);
    auto import_path = import_vis.ptr_->m_path;
    if (this->m_import_table.find(import_path->m_value)!=this->m_import_table.end()) {
      continue;
    }

    std::string import_file_path;
    bool found = false;
    for (const auto &search_path : this->m_dal_search_paths) {
      std::error_code ec2;
      auto maybe_import = os::join_path(search_path, import_path->m_value + ".dal", ec);
      if (ec2) {
        continue;
      }

      if (os::is_file(maybe_import, ec2)) {
        import_file_path = maybe_import;
        found = true;
        break;
      }
    }

    if (!found) {
      fmt::panic("Failed to find import: {}", import_path->m_value);
    }

    this->add_code(import_file_path);
  }

  return res_ast;
}
void codegen::add_error(ast *node, const std::string &msg) {
  error e(msg, node->m_span, node->m_owner.lock()->get_path(), node->m_owner.lock()->get_source());
  this->m_errors.push_back(e);
}

block_ctx::block_ctx(std::shared_ptr<ast> node, std::shared_ptr<block_ctx> parent) {
  if (parent) {
    this->m_root = parent->m_root;
  } else {
    this->m_root = std::shared_ptr<block_ctx>(this);
  }
  this->m_node = std::move(node);
  this->m_parent = std::move(parent);

  auto fn_def_node = this->m_root->m_node;
  auto fn_def_info = fn_def_node->m_cg_node->m_fn_def;
  fn_def_info.blocks.push_back(std::shared_ptr<block_ctx>(this));
}

void block_ctx::set_node(const std::shared_ptr<ast> &node) {
  this->m_node = node;
}

void block_ctx::set_root(const std::shared_ptr<block_ctx> &root) {
  this->m_root = root;
}

void block_ctx::set_parent(const std::shared_ptr<block_ctx> &parent) {
  this->m_parent = parent;
}

void block_ctx::add_local_var(const std::string &name, const std::shared_ptr<local_var_table> &table) {
  this->m_local_vars.insert({name, table});
}

std::shared_ptr<ast> block_ctx::get_node() {
  return this->m_node;
}

std::shared_ptr<block_ctx> block_ctx::get_parent() {
  return this->m_parent;
}

std::shared_ptr<block_ctx> block_ctx::get_root() {
  return this->m_root;
}

std::shared_ptr<local_var_table> block_ctx::get_local_var(const std::string &name) {
  auto it = this->m_local_vars.find(name);
  if (it!=this->m_local_vars.end()) {
    return it->second;
  }

  return nullptr;
}

bool block_ctx::has_local_var(const std::string &name) const {
  return this->m_local_vars.find(name)!=this->m_local_vars.end();
}

}  // namespace dal::core