/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/parser.hh>

namespace dal::core {

parser::parser(const std::string &source, const std::vector<token> &tokens) {
  this->m_source = source;
  this->m_tokens = tokens;
}

std::unique_ptr<ast> parser::parse() {
  auto root = std::make_unique<root_ast>();

  for (;;) {
    token *tok = &this->m_tokens[this->m_index];
    this->parse_attrs();

    auto fn_def = this->parse_fn_def(false);
    if (fn_def) {
      root->add_child(std::move(fn_def));
      continue;
    }

    auto extern_node = this->parse_extern(false);
    if (extern_node) {
      root->add_child(std::move(extern_node));
      continue;
    }

    auto import = this->parse_import(false);
    if (import) {
      root->add_child(std::move(import));
      continue;
    }

    if (!this->m_attrs.empty()) {
      this->error(*tok, "Unexpected attribute");
    }

    break;
  }

  if (this->m_index < this->m_tokens.size() - 1) {
    this->error(this->m_tokens[this->m_index], "Unexpected token: " + this->m_tokens[this->m_index].t_kind_str());
  }

  return root;
}

void parser::parse_attrs() {
  for (;;) {
    token *tok = &this->m_tokens[this->m_index];
    if (tok->t_kind()==token_kind::punct_at) {
      this->parse_attr();
    } else {
      return;
    }
  }
}

void parser::parse_attr() {
  token *tok = &this->m_tokens[this->m_index];
  this->expect(*tok, token_kind::punct_at);
  this->m_index++;

  token *name_tok = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*name_tok, token_kind::ident);

  token *lparen_tok = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*lparen_tok, token_kind::punct_lparen);

  token *value_tok = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*value_tok, token_kind::lit_string);

  token *rparen_tok = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*rparen_tok, token_kind::punct_rparen);

  auto attr = std::make_unique<attr_ast>();
  attr->set_name(this->tok_value(*name_tok));
  attr->set_arg(this->tok_value(*value_tok));
  this->m_attrs.push_back(std::move(attr));
}

std::unique_ptr<type_ast> parser::create_prim_type(const span &type_span, const std::string &type_name) {
  auto type = std::make_unique<type_ast>();
  type->set_span(type_span);
  auto ident = std::make_unique<ident_ast>();
  ident->set_name(type_name);
  type->set_type_name(std::move(ident));
  return type;
}

std::string parser::tok_value(const token &tok) const {
  return this->m_source.substr(tok.t_span().start_pos(), tok.t_span().len());
}

std::unique_ptr<ident_ast> parser::parse_ident(bool is_required) {
  token *tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::ident) {
    if (is_required) {
      this->error(*tok, "Expected identifier");
    } else {
      return nullptr;
    }
  }
  this->m_index++;

  auto ident = std::make_unique<ident_ast>();
  ident->set_name(this->tok_value(*tok));
  ident->set_span(tok->t_span());
  return ident;
}

std::unique_ptr<import_ast> parser::parse_import(bool is_required) {
  token *tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_import) {
    if (is_required) {
      this->error(*tok, "Expected import");
    } else {
      return nullptr;
    }
  }
  this->m_index++;

  auto path = this->parse_string();
  auto import = std::make_unique<import_ast>();
  import->set_path(std::move(path));
  import->set_span(tok->t_span());
  return import;
}

std::unique_ptr<string_ast> parser::parse_string() {
  auto tok = &this->m_tokens[this->m_index];
  this->expect(*tok, token_kind::lit_string);
  this->m_index++;

  span t_span = tok->t_span();
  bool escape = false;
  bool first = true;
  std::string value;
  for (int i = t_span.start_pos(); i < t_span.end_pos() - 1; i++) {
    char c = this->m_source[i];

    if (first) {
      first = false;
    } else {
      if (escape) {
        switch (c) {
        case '\\':value += '\\';
          break;
        case 'r':value += '\r';
          break;
        case 'n':value += '\n';
          break;
        case 't':value += '\t';
          break;
        case '"':value += '"';
          break;
        default:break;
        }
        escape = false;
      } else if (c=='\\') {
        escape = true;
      } else {
        value += c;
      }
    }
  }

  auto str = std::make_unique<string_ast>();
  str->set_span(t_span);
  str->set_value(value);
  return str;
}

std::unique_ptr<int_ast> parser::parse_int() {
  auto tok = &this->m_tokens[this->m_index];
  this->expect(*tok, token_kind::lit_int);
  this->m_index++;

  auto int_node = std::make_unique<int_ast>();
  int_node->set_span(tok->t_span());
  int_node->set_value(this->tok_value(*tok));
  return int_node;
}

std::unique_ptr<bool_ast> parser::parse_bool() {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_true && tok->t_kind()!=token_kind::kw_false) {
    this->error(*tok, "Expected boolean");
  }
  this->m_index++;

  auto bool_node = std::make_unique<bool_ast>();
  bool_node->set_span(tok->t_span());
  bool_node->set_value(tok->t_kind()==token_kind::kw_true);
  return bool_node;
}

std::unique_ptr<type_ast> parser::parse_type() {
  auto tok = &this->m_tokens[this->m_index];
  this->m_index++;

  auto type = std::make_unique<type_ast>();
  type->set_span(tok->t_span());

  if (tok->t_kind()==token_kind::op_log_not) {
    // no return type: !
    type = this->create_prim_type(tok->t_span(), "!");
  } else if (tok->t_kind()==token_kind::kw_void) {
    // void type: void
    type = this->create_prim_type(tok->t_span(), "void");
  } else if (tok->t_kind()==token_kind::ident) {
    // user defined or primitive type: u8, u16, etc.
    type = this->create_prim_type(tok->t_span(), this->tok_value(*tok));
  } else if (tok->t_kind()==token_kind::op_mul) {
    // pointer type: *const u8, *mut u8, etc.
    auto const_or_mut = &this->m_tokens[this->m_index];
    this->m_index++;
    bool is_mut;
    if (const_or_mut->t_kind()==token_kind::kw_mut) {
      is_mut = true;
    } else if (const_or_mut->t_kind()==token_kind::kw_const) {
      is_mut = false;
    } else {
      this->error(*const_or_mut, "Expected const or mut");
    }

    auto inner_type = this->parse_type();
    type->set_type_kind(type_kind::pointer);
    type->set_mutability(is_mut);
    type->set_type_child(std::move(inner_type));
  } else if (tok->t_kind()==token_kind::punct_lbracket) {
    // array type: [u8; 10].
    auto inner_type = this->parse_type();

    auto semicolon = &this->m_tokens[this->m_index];
    this->expect(*semicolon, token_kind::punct_semicolon);
    this->m_index++;

    auto size = this->parse_int();

    auto rbracket = &this->m_tokens[this->m_index];
    this->expect(*rbracket, token_kind::punct_rbracket);
    this->m_index++;

    type->set_type_kind(type_kind::array);
    type->set_type_child(std::move(inner_type));
    type->set_type_size(std::move(size));
  } else {
    this->error(*tok, "Expected type");
  }

  return type;
}

std::unique_ptr<extern_ast> parser::parse_extern(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_extern) {
    if (is_required) {
      this->error(*tok, "Expected extern");
    } else {
      return nullptr;
    }
  }
  this->m_index++;

  auto extern_node = std::make_unique<extern_ast>();
  extern_node->set_span(tok->t_span());
  extern_node->set_attrs(std::move(this->m_attrs));
  this->m_attrs.clear();

  auto lb = &this->m_tokens[this->m_index];
  this->expect(*lb, token_kind::punct_lbrace);
  this->m_index++;

  for (;;) {
    auto maybe_attr = &this->m_tokens[this->m_index];
    this->parse_attrs();

    auto maybe_rbrace = &this->m_tokens[this->m_index];
    if (maybe_rbrace->t_kind()==token_kind::punct_rbrace) {
      if (!this->m_attrs.empty()) {
        this->error(*maybe_attr, "Unexpected attribute");
      }
      this->m_attrs.clear();
      this->m_index++;
      return extern_node;
    } else {
      auto fn_decl = this->parse_fn_decl();
      extern_node->add_fn(std::move(fn_decl));
    }
  }
}

std::unique_ptr<void_ast> parser::create_void_ast(const span &void_span) {
  auto void_node = std::make_unique<void_ast>();
  void_node->set_span(void_span);
  return void_node;
}

std::unique_ptr<no_ret_ast> parser::create_no_ret_ast(const span &no_ret_span) {
  auto no_ret_node = std::make_unique<no_ret_ast>();
  no_ret_node->set_span(no_ret_span);
  return no_ret_node;
}

void parser::error(const token &tok, const std::string &msg) {
  dal::core::error e(msg, tok.t_span());
  e.raise(this->m_source);
}

void parser::error(const span &span, const std::string &msg) {
  dal::core::error e(msg, span);
  e.raise(this->m_source);
}

void parser::expect(const token &tok, token_kind kind) {
  if (tok.t_kind()!=kind) {
    token dummy(kind, tok.t_span());
    this->error(tok, "Expected " + dummy.t_kind_str());
  }
}

std::unique_ptr<var_decl_ast> parser::parse_var_decl(bool is_required) {
  auto let = &this->m_tokens[this->m_index];
  if (let->t_kind()!=token_kind::kw_let) {
    if (is_required) {
      this->error(*let, "Expected var");
    } else {
      return nullptr;
    }
  }
  this->m_index++;

  auto var_decl = std::make_unique<var_decl_ast>();
  var_decl->set_span(let->t_span());

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()==token_kind::kw_mut) {
    var_decl->set_mutability(true);
    this->m_index++;
    auto name = this->parse_ident(true);
    var_decl->set_name(std::move(name));
  } else if (tok->t_kind()==token_kind::ident) {
    var_decl->set_mutability(false);
    auto name = this->parse_ident(true);
    var_decl->set_name(std::move(name));
  } else {
    this->error(*tok, "Expected var name");
  }

  auto maybe_colon = &this->m_tokens[this->m_index];
  if (maybe_colon->t_kind()==token_kind::op_assign) {
    this->m_index++;
    auto expr = this->parse_expr(true);
    var_decl->set_value(std::move(expr));
    return var_decl;
  } else if (maybe_colon->t_kind()==token_kind::punct_colon) {
    this->m_index++;
    auto type = this->parse_type();
    var_decl->set_type(std::move(type));
    return var_decl;
  } else {
    this->error(*maybe_colon, "Expected : or =");
  }
}

} // namespace dal::core