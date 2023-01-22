/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/parser.hh>

namespace dal::core {

std::unique_ptr<fn_def_ast> parser::parse_fn_def(bool is_required) {
  auto proto = this->parse_fn_proto(is_required);
  if (!proto) {
    return nullptr;
  }

  auto block = this->parse_block(true);

  auto fn_def = std::make_unique<fn_def_ast>();
  fn_def->set_span(proto->ast_span());
  fn_def->set_proto(std::move(proto));
  fn_def->set_block(std::move(block));
  return fn_def;
}

std::unique_ptr<fn_proto_ast> parser::parse_fn_proto(bool is_required) {
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

  std::unique_ptr<type_ast> ret_type;

  token *maybe_arrow = &this->m_tokens[this->m_index];
  if (maybe_arrow->t_kind()==token_kind::punct_arrow) {
    this->m_index++;
    ret_type = this->parse_type();
  } else {
    ret_type = parser::create_prim_type(maybe_arrow->t_span(), "void");
  }

  auto fn_proto = std::make_unique<fn_proto_ast>();
  fn_proto->set_span(tok->t_span());
  fn_proto->set_public(is_pub);
  fn_proto->set_variadic(is_variadic);
  fn_proto->set_name(std::move(fn_name));
  fn_proto->set_params(std::move(fn_params));
  fn_proto->set_return_type(std::move(ret_type));
  fn_proto->set_attrs(std::move(this->m_attrs));
  this->m_attrs.clear();
  return fn_proto;
}

std::vector<std::unique_ptr<fn_param_ast>> parser::parse_fn_params(bool *is_variadic) {
  auto lp = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*lp, token_kind::punct_lparen);

  auto maybe_rp = &this->m_tokens[this->m_index];
  if (maybe_rp->t_kind()==token_kind::punct_rparen) {
    this->m_index++;
    return {};
  }

  std::vector<std::unique_ptr<fn_param_ast>> params;
  for (;;) {
    auto param = this->parse_fn_param();
    bool expect_end = false;
    if (param) {
      params.push_back(std::move(param));
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

std::unique_ptr<fn_param_ast> parser::parse_fn_param() {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()==token_kind::ident) {
    auto ident = this->parse_ident(true);

    auto colon = &this->m_tokens[this->m_index];
    this->m_index++;
    this->expect(*colon, token_kind::punct_colon);

    auto type = this->parse_type();

    auto param = std::make_unique<fn_param_ast>();
    param->set_span(tok->t_span());
    param->set_name(std::move(ident));
    param->set_type(std::move(type));
    return param;
  } else if (tok->t_kind()==token_kind::punct_ellipsis) {
    this->m_index++;
    return nullptr;
  } else {
    this->error(*tok, "Expected identifier or '...'");
  }
}

std::unique_ptr<fn_decl_ast> parser::parse_fn_decl() {
  auto proto = this->parse_fn_proto(true);
  auto decl = std::make_unique<fn_decl_ast>();
  decl->set_span(proto->ast_span());
  decl->set_proto(std::move(proto));
  return decl;
}

std::vector<std::unique_ptr<ast>> parser::parse_call_args() {
  auto lp = &this->m_tokens[this->m_index];
  this->m_index++;
  this->expect(*lp, token_kind::punct_lparen);

  auto maybe_rp = &this->m_tokens[this->m_index];
  if (maybe_rp->t_kind()==token_kind::punct_rparen) {
    this->m_index++;
    return {};
  }

  std::vector<std::unique_ptr<ast>> args;
  for (;;) {
    auto expr = this->parse_expr(true);
    args.push_back(std::move(expr));

    auto tok = &this->m_tokens[this->m_index];
    this->m_index++;
    if (tok->t_kind()==token_kind::punct_rparen) {
      return args;
    } else {
      this->expect(*tok, token_kind::punct_comma);
    }
  }
}

} // namespace dal::core