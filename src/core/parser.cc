/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/parser.hh>
#include <core/visitor.hh>

namespace dal::core {

parser::parser(const std::string &source, const std::vector<token> &tokens,
               std::shared_ptr<import_table> owner) {
  this->m_source = source;
  this->m_owner = std::move(owner);
  this->m_tokens = tokens;
}

ast *parser::parse() {
  auto root = new root_ast();
  root->m_owner = this->m_owner;

  auto expect_module = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*expect_module, token_kind::kw_module);

  auto module_name = this->parse_ident(true);
  root->m_module_name = std::shared_ptr<ident_ast>(module_name);

  for (;;) {
    token *tok = &this->m_tokens[this->m_index];
    this->parse_attrs();

    auto fn_def = this->parse_fn_def(false);
    if (fn_def) {
      root->m_children.push_back(std::shared_ptr<ast>(fn_def));
      continue;
    }

    auto extern_node = this->parse_extern(false);
    if (extern_node) {
      root->m_children.push_back(std::shared_ptr<ast>(extern_node));
      continue;
    }

    auto import = this->parse_import(false);
    if (import) {
      root->m_children.push_back(std::shared_ptr<ast>(import));
      continue;
    }

    if (!this->m_attrs.empty()) {
      this->error(*tok, "Unexpected attribute");
    }

    break;
  }

  if (this->m_index < this->m_tokens.size() - 1) {
    this->error(
        this->m_tokens[this->m_index],
        "Unexpected token: " + this->m_tokens[this->m_index].t_kind_str());
  }

  return root;
}

type_ast *parser::create_prim_type(const span &type_span,
                                   const std::string &type_name) {
  auto type = new type_ast();
  type->m_span = type_span;
  auto ident = new ident_ast();
  ident->m_name = type_name;
  type->m_primitive = std::shared_ptr<ident_ast>(ident);
  type->m_owner = this->m_owner;
  return type;
}

ident_ast *parser::parse_ident(bool is_required) {
  token *tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::ident) {
    if (is_required) {
      this->error(*tok, "Expected identifier");
    } else {
      return nullptr;
    }
  }
  this->m_index++;

  auto ident = new ident_ast();
  ident->m_name = this->tok_value(*tok);
  ident->m_span = tok->t_span();
  ident->m_owner = this->m_owner;
  return ident;
}

import_ast *parser::parse_import(bool is_required) {
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
  auto import = new import_ast();
  import->m_path = std::shared_ptr<string_ast>(path);
  import->m_span = tok->t_span();
  import->m_owner = this->m_owner;
  return import;
}

string_ast *parser::parse_string() {
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

  auto str = new string_ast();
  str->m_span = t_span;
  str->m_value = value;
  str->m_owner = this->m_owner;
  return str;
}

int_ast *parser::parse_int() {
  auto tok = &this->m_tokens[this->m_index];
  this->expect(*tok, token_kind::lit_int);
  this->m_index++;

  auto int_node = new int_ast();
  int_node->m_span = tok->t_span();
  int_node->m_value = this->tok_value(*tok);
  int_node->m_owner = this->m_owner;
  return int_node;
}

bool_ast *parser::parse_bool() {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_true &&
      tok->t_kind()!=token_kind::kw_false) {
    this->error(*tok, "Expected boolean");
  }
  this->m_index++;

  auto bool_node = new bool_ast();
  bool_node->m_span = tok->t_span();
  bool_node->m_value = tok->t_kind()==token_kind::kw_true;
  bool_node->m_owner = this->m_owner;
  return bool_node;
}

type_ast *parser::parse_type() {
  auto tok = &this->m_tokens[this->m_index];
  this->m_index++;

  type_ast *type = nullptr;

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
    bool is_mut = false;
    if (const_or_mut->t_kind()==token_kind::kw_mut) {
      is_mut = true;
    } else if (const_or_mut->t_kind()==token_kind::kw_const) {
      is_mut = false;
    } else {
      this->error(*const_or_mut, "Expected const or mut");
    }

    auto inner_type = this->parse_type();
    type = new type_ast();
    type->m_span = tok->t_span();
    type->m_type_kind = type_kind::pointer;
    type->m_is_mut = is_mut;
    type->m_child = std::shared_ptr<type_ast>(inner_type);
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

    type = new type_ast();
    type->m_span = tok->t_span();
    type->m_type_kind = type_kind::array;
    type->m_child = std::shared_ptr<type_ast>(inner_type);
    type->m_size = std::shared_ptr<int_ast>(size);
  } else {
    this->error(*tok, "Expected type");
  }

  type->m_owner = this->m_owner;
  return type;
}

extern_ast *parser::parse_extern(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_extern) {
    if (is_required) {
      this->error(*tok, "Expected extern");
    } else {
      return nullptr;
    }
  }
  this->m_index++;

  auto extern_node = new extern_ast();
  extern_node->m_span = tok->t_span();
  extern_node->m_attrs = std::move(this->m_attrs);
  extern_node->m_owner = this->m_owner;
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
      extern_node->m_fns.push_back(std::shared_ptr<fn_decl_ast>(fn_decl));
    }
  }
}

void_ast *parser::create_void_ast(const span &void_span) {
  auto void_node = new void_ast();
  void_node->m_span = void_span;
  void_node->m_owner = this->m_owner;
  return void_node;
}

no_ret_ast *parser::create_no_ret_ast(const span &no_ret_span) {
  auto no_ret_node = new no_ret_ast();
  no_ret_node->m_span = no_ret_span;
  no_ret_node->m_owner = this->m_owner;
  return no_ret_node;
}

var_decl_ast *parser::parse_var_decl(bool is_required) {
  auto let = &this->m_tokens[this->m_index];
  if (let->t_kind()!=token_kind::kw_let) {
    if (is_required) {
      this->error(*let, "Expected var");
    } else {
      return nullptr;
    }
  }
  this->m_index++;

  auto var_decl = new var_decl_ast();
  var_decl->m_span = let->t_span();
  var_decl->m_owner = this->m_owner;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()==token_kind::kw_mut) {
    var_decl->m_is_mut = true;
    this->m_index++;
    auto name = this->parse_ident(true);
    var_decl->m_name = std::shared_ptr<ident_ast>(name);
  } else if (tok->t_kind()==token_kind::ident) {
    var_decl->m_is_mut = false;
    auto name = this->parse_ident(true);
    var_decl->m_name = std::shared_ptr<ident_ast>(name);
  } else {
    this->error(*tok, "Expected var name");
  }

  auto maybe_colon = &this->m_tokens[this->m_index];
  if (maybe_colon->t_kind()==token_kind::op_assign) {
    this->m_index++;
    auto expr = this->parse_expr(true);
    var_decl->m_value = std::shared_ptr<ast>(expr);
    return var_decl;
  } else if (maybe_colon->t_kind()==token_kind::punct_colon) {
    this->m_index++;
    auto type = this->parse_type();
    var_decl->m_type = std::shared_ptr<type_ast>(type);
    return var_decl;
  } else {
    this->error(*maybe_colon, "Expected : or =");
  }
}

block_ast *parser::parse_block(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];

  if (tok->t_kind()!=token_kind::punct_lbrace) {
    if (is_required) {
      this->error(*tok, "Expected '{'");
    }
    return nullptr;
  }
  this->m_index++;

  auto block = new block_ast();
  block->m_span = tok->t_span();
  block->m_owner = this->m_owner;

  for (;;) {
    ast *stmt = this->parse_var_decl(false);
    if (!stmt)
      stmt = this->parse_if_or_block(false);
    if (!stmt)
      stmt = this->parse_return_or_assign(false);
    if (!stmt)
      stmt = parser::create_void_ast(tok->t_span());

    block->m_children.push_back(std::shared_ptr<ast>(stmt));

    tok = &this->m_tokens[this->m_index];
    if (tok->t_kind()==token_kind::punct_rbrace) {
      this->m_index++;
      return block;
    } else if (tok->t_kind()==token_kind::punct_semicolon) {
      // FIXME: this is to avoid infinite loop when finding `;`.
      this->error(*tok, "Unexpected ';'");
    }
  }
}

if_ast *parser::parse_if(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_if) {
    if (is_required) {
      this->error(*tok, "Expected if expression");
    }
    return nullptr;
  }
  this->m_index++;

  auto if_node = new if_ast();
  if_node->m_span = tok->t_span();
  if_node->m_owner = this->m_owner;

  auto cond = this->parse_expr(true);
  if_node->m_cond = std::shared_ptr<ast>(cond);
  auto block = this->parse_block(true);
  if_node->m_then = std::shared_ptr<block_ast>(block);
  auto else_node = this->parse_else(false);
  if (else_node)
    if_node->m_else = std::shared_ptr<ast>(else_node);

  return if_node;
}

return_ast *parser::parse_return(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_return) {
    if (is_required) {
      this->error(*tok, "Expected `return`");
    }
    return nullptr;
  }
  this->m_index++;

  auto ret_node = new return_ast();
  ret_node->m_span = tok->t_span();
  auto value = this->parse_expr(false);
  if (value)
    ret_node->m_value = std::shared_ptr<ast>(value);

  ret_node->m_owner = this->m_owner;
  return ret_node;
}

fn_def_ast *parser::parse_fn_def(bool is_required) {
  auto proto = this->parse_fn_proto(is_required);
  if (!proto) {
    return nullptr;
  }

  auto block = this->parse_block(true);

  auto fn_def = new fn_def_ast();
  fn_def->m_span = proto->m_span;
  fn_def->m_proto = std::shared_ptr<fn_proto_ast>(proto);
  fn_def->m_body = std::shared_ptr<block_ast>(block);
  fn_def->m_owner = this->m_owner;
  return fn_def;
}

fn_proto_ast *parser::parse_fn_proto(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  bool is_pub = false;

  if (tok->t_kind()==token_kind::kw_pub) {
    is_pub = true;
    this->m_index++;
    auto fn_tok = &this->m_tokens[this->m_index];
    this->m_index++;
    this->expect(*fn_tok, token_kind::kw_fn);
  } else if (tok->t_kind()==token_kind::kw_fn) {
    this->m_index++;
  } else {
    if (is_required) {
      this->error(*tok, "Expected 'fn' or 'pub fn'");
    }
    return nullptr;
  }

  auto fn_name = this->parse_ident(true);
  bool is_variadic = false;
  auto fn_params = this->parse_fn_params(&is_variadic);

  type_ast *ret_type;

  token *maybe_arrow = &this->m_tokens[this->m_index];
  if (maybe_arrow->t_kind()==token_kind::punct_arrow) {
    this->m_index++;
    ret_type = this->parse_type();
  } else {
    ret_type = this->create_prim_type(maybe_arrow->t_span(), "void");
  }

  auto fn_proto = new fn_proto_ast();
  fn_proto->m_span = tok->t_span();
  fn_proto->m_is_pub = is_pub;
  fn_proto->m_is_variadic = is_variadic;
  fn_proto->m_name = std::shared_ptr<ident_ast>(fn_name);
  fn_proto->m_params = std::move(fn_params);
  fn_proto->m_return_type = std::shared_ptr<type_ast>(ret_type);
  fn_proto->m_attrs = std::move(this->m_attrs);
  fn_proto->m_owner = this->m_owner;
  this->m_attrs.clear();
  return fn_proto;
}

fn_param_ast *parser::parse_fn_param() {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()==token_kind::ident) {
    auto ident = this->parse_ident(true);

    auto colon = &this->m_tokens[this->m_index];
    this->m_index++;
    this->expect(*colon, token_kind::punct_colon);

    auto type = this->parse_type();

    auto param = new fn_param_ast();
    param->m_span = tok->t_span();
    param->m_name = std::shared_ptr<ident_ast>(ident);
    param->m_type = std::shared_ptr<type_ast>(type);
    param->m_owner = this->m_owner;
    return param;
  } else if (tok->t_kind()==token_kind::punct_ellipsis) {
    this->m_index++;
    return nullptr;
  } else {
    this->error(*tok, "Expected identifier or '...'");
  }
}

fn_decl_ast *parser::parse_fn_decl() {
  auto proto = this->parse_fn_proto(true);
  auto decl = new fn_decl_ast();
  decl->m_span = proto->m_span;
  decl->m_proto = std::shared_ptr<fn_proto_ast>(proto);
  decl->m_owner = this->m_owner;
  return decl;
}

ast *parser::parse_if_or_block(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];

  auto if_node = this->parse_if(false);
  if (if_node)
    return if_node;

  auto block = this->parse_block(false);
  if (block)
    return block;

  if (is_required)
    this->error(*tok, "Expected block expression");

  return nullptr;
}

ast *parser::parse_else(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_else) {
    if (is_required) {
      this->error(*tok, "Expected `else` expression");
    }
    return nullptr;
  }
  this->m_index++;

  auto if_node = this->parse_if(false);
  if (if_node)
    return if_node;

  return this->parse_block(true);
}

ast *parser::parse_return_or_assign(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];

  auto ret_node = this->parse_return(false);
  if (ret_node)
    return ret_node;

  auto assign = this->parse_assign(false);
  if (assign)
    return assign;

  if (is_required)
    this->error(*tok, "Expected expression");

  return nullptr;
}

// x = 1 | x = 1 + 2 | x = add(1, 2)
ast *parser::parse_assign(bool is_required) {
  auto lhs = this->parse_log_or(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_assign)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_log_or(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = bin_op_kind::op_assign;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// x || y
ast *parser::parse_log_or(bool is_required) {
  auto lhs = this->parse_log_and(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_log_or)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_log_and(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = bin_op_kind::op_log_or;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// x && y
ast *parser::parse_log_and(bool is_required) {
  auto lhs = this->parse_comparison(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_log_and)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_comparison(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = bin_op_kind::op_log_and;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// x == y | x != y | x < y | x > y | x <= y | x >= y
ast *parser::parse_comparison(bool is_required) {
  auto lhs = this->parse_bit_or(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  bin_op_kind op;
  switch (tok->t_kind()) {
    case token_kind::op_eq:op = bin_op_kind::op_eq;
      break;
    case token_kind::op_neq:op = bin_op_kind::op_neq;
      break;
    case token_kind::op_lt:op = bin_op_kind::op_lt;
      break;
    case token_kind::op_gt:op = bin_op_kind::op_gt;
      break;
    case token_kind::op_lte:op = bin_op_kind::op_lte;
      break;
    case token_kind::op_gte:op = bin_op_kind::op_gte;
      break;
    default:return lhs;
  }
  this->m_index++;

  auto rhs = this->parse_bit_or(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = op;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// x | y
ast *parser::parse_bit_or(bool is_required) {
  auto lhs = this->parse_bit_xor(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_or)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_bit_xor(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = bin_op_kind::op_or;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// x ^ y
ast *parser::parse_bit_xor(bool is_required) {
  auto lhs = this->parse_bit_and(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_xor)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_bit_and(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = bin_op_kind::op_xor;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// x & y
ast *parser::parse_bit_and(bool is_required) {
  auto lhs = this->parse_bit_shift(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_and)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_bit_shift(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = bin_op_kind::op_and;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// x << y | x >> y
ast *parser::parse_bit_shift(bool is_required) {
  auto lhs = this->parse_add(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  bin_op_kind op;
  switch (tok->t_kind()) {
    case token_kind::op_shl:op = bin_op_kind::op_shl;
      break;
    case token_kind::op_shr:op = bin_op_kind::op_shr;
      break;
    default:return lhs;
  }
  this->m_index++;

  auto rhs = this->parse_add(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = op;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// x + y | x - y
ast *parser::parse_add(bool is_required) {
  auto lhs = this->parse_mul(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  bin_op_kind op;
  switch (tok->t_kind()) {
    case token_kind::op_add:op = bin_op_kind::op_add;
      break;
    case token_kind::op_sub:op = bin_op_kind::op_sub;
      break;
    default:return lhs;
  }
  this->m_index++;

  auto rhs = this->parse_mul(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = op;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// x * y | x / y | x % y
ast *parser::parse_mul(bool is_required) {
  auto lhs = this->parse_cast(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  bin_op_kind op;
  switch (tok->t_kind()) {
    case token_kind::op_mul:op = bin_op_kind::op_mul;
      break;
    case token_kind::op_div:op = bin_op_kind::op_div;
      break;
    case token_kind::op_mod:op = bin_op_kind::op_mod;
      break;
    default:return lhs;
  }
  this->m_index++;

  auto rhs = this->parse_cast(true);

  auto bin_op = new bin_op_ast();
  bin_op->m_span = tok->t_span();
  bin_op->m_lhs = std::shared_ptr<ast>(lhs);
  bin_op->m_rhs = std::shared_ptr<ast>(rhs);
  bin_op->m_op = op;
  bin_op->m_owner = this->m_owner;

  return bin_op;
}

// 10 as u8 | x as u8
ast *parser::parse_cast(bool is_required) {
  auto lhs = this->parse_unary(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_as)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_type();

  auto cast = new cast_ast();
  cast->m_span = tok->t_span();
  cast->m_value = std::shared_ptr<ast>(lhs);
  cast->m_type = std::shared_ptr<type_ast>(rhs);
  cast->m_owner = this->m_owner;

  return cast;
}

// -x | !x | ~x
ast *parser::parse_unary(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  un_op_kind op;
  switch (tok->t_kind()) {
    case token_kind::op_sub:op = un_op_kind::op_neg;
      break;
    case token_kind::op_not:op = un_op_kind::op_not;
      break;
    case token_kind::op_log_not:op = un_op_kind::op_log_not;
      break;
    default:return this->parse_postfix(is_required);
  }
  this->m_index++;

  auto rhs = this->parse_postfix(true);

  auto un_op = new un_op_ast();
  un_op->m_span = tok->t_span();
  un_op->m_value = std::shared_ptr<ast>(rhs);
  un_op->m_op = op;
  un_op->m_owner = this->m_owner;

  return un_op;
}

// x() | x[y]
ast *parser::parse_postfix(bool is_required) {
  auto primary = this->parse_primary(is_required);
  if (!primary)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()==token_kind::punct_dot) {
    // x.y: member access
    this->m_index++;
    auto access = this->parse_postfix(true);
    // for now we only support accessing functions
    if (access->kind()!=ast_kind::call_node) {
      this->error(access->m_span, "expected call");
      return nullptr;
    }
    auto node = new member_access_ast();
    node->m_span = primary->m_span;
    ast_visitor::visitor_ptr<ident_ast> v;
    primary->accept(v);
    node->m_name = std::shared_ptr<ident_ast>(v.ptr_);
    ast_visitor::visitor_ptr<call_ast> v2;
    access->accept(v2);
    node->m_call = std::shared_ptr<call_ast>(v2.ptr_);
    return node;
  } else if (tok->t_kind()==token_kind::punct_lparen) {
    // x(): function call
    auto args = this->parse_call_args();
    auto call = new call_ast();
    auto ast_span = primary->m_span;
    call->m_span = ast_span;
    if (primary->kind()!=ast_kind::ident_node)
      this->error(ast_span, "expected identifier");
    ast_visitor::visitor_ptr<ident_ast> v;
    primary->accept(v);
    call->m_name = std::shared_ptr<ident_ast>(v.ptr_);
    call->m_args = std::move(args);
    return call;
  } else if (tok->t_kind()==token_kind::punct_lbracket) {
    // x[y]: array index
    this->m_index++;

    auto index = this->parse_expr(true);

    auto rb = &this->m_tokens[this->m_index];
    this->expect(*rb, token_kind::punct_rbracket);
    this->m_index++;

    auto index_ast = new array_index_ast();
    index_ast->m_span = tok->t_span();

    if (primary->kind()!=ast_kind::ident_node)
      this->error(tok->t_span(), "expected identifier");
    ast_visitor::visitor_ptr<ident_ast> v;
    primary->accept(v);
    index_ast->m_array = std::shared_ptr<ident_ast>(v.ptr_);
    index_ast->m_index = std::shared_ptr<ast>(index);

    return index_ast;
  } else {
    return primary;
  }
}

ast *parser::parse_primary(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];

  if (tok->t_kind()==token_kind::lit_int) {
    return this->parse_int();
  } else if (tok->t_kind()==token_kind::lit_string) {
    return this->parse_string();
  } else if (tok->t_kind()==token_kind::op_log_not) {
    auto no_ret = this->create_no_ret_ast(tok->t_span());
    this->m_index++;
    return no_ret;
  } else if (tok->t_kind()==token_kind::kw_void) {
    auto void_ast = this->create_void_ast(tok->t_span());
    this->m_index++;
    return void_ast;
  } else if (tok->t_kind()==token_kind::kw_true ||
      tok->t_kind()==token_kind::kw_false) {
    return this->parse_bool();
  } else if (tok->t_kind()==token_kind::ident) {
    return this->parse_ident(true);
  }

  // group: (expr)
  auto group = this->parse_group(false);
  if (group)
    return group;

  if (is_required)
    this->error(tok->t_span(), "expected expression");

  return nullptr;
}

ast *parser::parse_group(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::punct_lparen) {
    if (is_required)
      this->error(tok->t_span(), "expected expression");
    return nullptr;
  }
  this->m_index++;

  auto expr = this->parse_expr(true);

  auto rp = &this->m_tokens[this->m_index];
  this->expect(*rp, token_kind::punct_rparen);
  this->m_index++;

  return expr;
}

ast *parser::parse_expr(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];

  auto if_or_block = this->parse_if_or_block(false);
  if (if_or_block)
    return if_or_block;

  auto return_or_assign = this->parse_return_or_assign(false);
  if (return_or_assign)
    return return_or_assign;

  if (is_required)
    this->error(tok->t_span(), "expected expression");

  return nullptr;
}

std::vector<std::shared_ptr<fn_param_ast>> parser::parse_fn_params(
    bool *is_variadic) {
  auto lp = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*lp, token_kind::punct_lparen);

  auto maybe_rp = &this->m_tokens[this->m_index];
  if (maybe_rp->t_kind()==token_kind::punct_rparen) {
    this->m_index++;
    return {};
  }

  std::vector<std::shared_ptr<fn_param_ast>> params;
  for (;;) {
    auto param = this->parse_fn_param();
    bool expect_end = false;
    if (param) {
      params.push_back(std::shared_ptr<fn_param_ast>(param));
    } else {
      expect_end = true;
      *is_variadic = true;
    }

    token *tok = &this->m_tokens[this->m_index];
    this->m_index++;
    if (tok->t_kind()==token_kind::punct_rparen) {
      return params;
    } else if (expect_end) {
      this->error(*tok, "Expected ')' after parameter");
    } else {
      this->expect(*tok, token_kind::punct_comma);
    }
  }
}

std::vector<std::shared_ptr<ast>> parser::parse_call_args() {
  auto lp = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*lp, token_kind::punct_lparen);

  auto maybe_rp = &this->m_tokens[this->m_index];
  if (maybe_rp->t_kind()==token_kind::punct_rparen) {
    this->m_index++;
    return {};
  }

  std::vector<std::shared_ptr<ast>> args;
  for (;;) {
    auto expr = this->parse_expr(true);
    args.push_back(std::shared_ptr<ast>(expr));

    auto tok = &this->m_tokens[this->m_index];
    this->m_index++;
    if (tok->t_kind()==token_kind::punct_rparen) {
      return args;
    } else {
      this->expect(*tok, token_kind::punct_comma);
    }
  }
}

[[noreturn]] static void unreachable() {
  std::abort();
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
  // attr := @[ident("arg")]
  // | @[ident, ident("arg")]
  // | @[ident]
  token *at = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*at, token_kind::punct_at);

  token *lb = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*lb, token_kind::punct_lbracket);

  for (;;) {
    token *tok = &this->m_tokens[this->m_index];
    if (tok->t_kind()==token_kind::ident) {
      this->m_index++;
      auto attr = new attr_ast();
      attr->m_span = tok->t_span();
      attr->m_name = this->tok_value(*tok);

      token *maybe_lp = &this->m_tokens[this->m_index];
      if (maybe_lp->t_kind()==token_kind::punct_lparen) {
        this->m_index++;
        token *str = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect(*str, token_kind::lit_string);
        attr->m_arg = this->tok_value(*str);

        token *rp = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect(*rp, token_kind::punct_rparen);
      }

      this->m_attrs.push_back(std::shared_ptr<attr_ast>(attr));
      tok = &this->m_tokens[this->m_index];
    }

    if (tok->t_kind()==token_kind::punct_rbracket) {
      this->m_index++;
      return;
    } else {
      this->expect(*tok, token_kind::punct_comma);
      this->m_index++;
    }
  }
}

void parser::error(const token &tok, const std::string &msg) {
  dal::core::error e(msg, tok.t_span(), this->m_owner->get_path(),
                     this->m_source);
  e.raise(true);
  unreachable();
}

void parser::error(const span &span, const std::string &msg) {
  dal::core::error e(msg, span, this->m_owner->get_path(), this->m_source);
  e.raise(true);
  unreachable();
}

void parser::expect(const token &tok, token_kind kind) {
  if (tok.t_kind()!=kind) {
    token dummy(kind, tok.t_span());
    this->error(tok, "Expected " + dummy.t_kind_str());
  }
}

std::string parser::tok_value(const token &tok) const {
  return this->m_source.substr(tok.t_span().start_pos(), tok.t_span().len());
}

}  // namespace dal::core