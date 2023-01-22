/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/parser.hh>

namespace dal::core {

std::unique_ptr<ast> parser::parse_return_or_assign(bool is_required) {
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

std::unique_ptr<return_ast> parser::parse_return(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_return) {
    if (is_required) {
      this->error(*tok, "Expected `return`");
    }
    return nullptr;
  }
  this->m_index++;

  auto ret_node = std::make_unique<return_ast>();
  ret_node->set_span(tok->t_span());
  auto value = this->parse_expr(false);
  if (value)
    ret_node->set_value(std::move(value));

  return ret_node;
}

// x = 1 | x = 1 + 2 | x = add(1, 2)
std::unique_ptr<ast> parser::parse_assign(bool is_required) {
  auto lhs = this->parse_log_or(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_assign)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_log_or(true);

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(bin_op_kind::op_assign);

  return bin_op;
}

// x || y
std::unique_ptr<ast> parser::parse_log_or(bool is_required) {
  auto lhs = this->parse_log_and(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_log_or)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_log_and(true);

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(bin_op_kind::op_log_or);

  return bin_op;
}

// x && y
std::unique_ptr<ast> parser::parse_log_and(bool is_required) {
  auto lhs = this->parse_comparison(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_log_and)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_comparison(true);

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(bin_op_kind::op_log_and);

  return bin_op;
}

// x == y | x != y | x < y | x > y | x <= y | x >= y
std::unique_ptr<ast> parser::parse_comparison(bool is_required) {
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

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(op);

  return bin_op;
}

// x | y
std::unique_ptr<ast> parser::parse_bit_or(bool is_required) {
  auto lhs = this->parse_bit_xor(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_or)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_bit_xor(true);

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(bin_op_kind::op_or);

  return bin_op;
}

// x ^ y
std::unique_ptr<ast> parser::parse_bit_xor(bool is_required) {
  auto lhs = this->parse_bit_and(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_xor)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_bit_and(true);

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(bin_op_kind::op_xor);

  return bin_op;
}

// x & y
std::unique_ptr<ast> parser::parse_bit_and(bool is_required) {
  auto lhs = this->parse_bit_shift(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::op_and)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_bit_shift(true);

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(bin_op_kind::op_and);

  return bin_op;
}

// x << y | x >> y
std::unique_ptr<ast> parser::parse_bit_shift(bool is_required) {
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

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(op);

  return bin_op;
}

// x + y | x - y
std::unique_ptr<ast> parser::parse_add(bool is_required) {
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

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(op);

  return bin_op;
}

// x * y | x / y | x % y
std::unique_ptr<ast> parser::parse_mul(bool is_required) {
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

  auto bin_op = std::make_unique<bin_op_ast>();
  bin_op->set_span(tok->t_span());
  bin_op->set_lhs(std::move(lhs));
  bin_op->set_rhs(std::move(rhs));
  bin_op->set_op(op);

  return bin_op;
}

// 10 as u8 | x as u8
std::unique_ptr<ast> parser::parse_cast(bool is_required) {
  auto lhs = this->parse_unary(is_required);
  if (!lhs)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_as)
    return lhs;
  this->m_index++;

  auto rhs = this->parse_type();

  auto cast = std::make_unique<cast_ast>();
  cast->set_span(tok->t_span());
  cast->set_value(std::move(lhs));
  cast->set_type(std::move(rhs));

  return cast;
}

// -x | !x | ~x
std::unique_ptr<ast> parser::parse_unary(bool is_required) {
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

  auto un_op = std::make_unique<un_op_ast>();
  un_op->set_span(tok->t_span());
  un_op->set_value(std::move(rhs));
  un_op->set_op(op);

  return un_op;
}

// x() | x[y]
std::unique_ptr<ast> parser::parse_postfix(bool is_required) {
  auto primary = this->parse_primary(is_required);
  if (!primary)
    return nullptr;

  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()==token_kind::punct_lparen) {
    // x(): function call
    auto args = this->parse_call_args();
    auto call = std::make_unique<call_ast>();
    auto ast_span = primary->ast_span();
    call->set_span(ast_span);
    auto ident = dynamic_cast<ident_ast *>(primary.get());
    if (!ident)
      this->error(ast_span, "expected identifier");
    call->set_ident(std::make_unique<ident_ast>(*ident));
    call->set_args(std::move(args));
    return call;
  } else if (tok->t_kind()==token_kind::punct_lbracket) {
    // x[y]: array index
    this->m_index++;

    auto index = this->parse_expr(true);

    auto rb = &this->m_tokens[this->m_index];
    this->expect(*rb, token_kind::punct_rbracket);
    this->m_index++;

    auto index_ast = std::make_unique<array_index_ast>();
    index_ast->set_span(tok->t_span());

    // TODO: allow array index from expression
    // e.g. call()[0]
    auto ident = dynamic_cast<ident_ast *>(primary.get());
    if (!ident)
      this->error(tok->t_span(), "expected identifier");
    index_ast->set_array(std::make_unique<ident_ast>(*ident));
    index_ast->set_index(std::move(index));

    return index_ast;
  } else {
    return primary;
  }
}

std::unique_ptr<ast> parser::parse_primary(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];

  if (tok->t_kind()==token_kind::lit_int) {
    return this->parse_int();
  } else if (tok->t_kind()==token_kind::lit_string) {
    return this->parse_string();
  } else if (tok->t_kind()==token_kind::op_log_not) {
    auto no_ret = parser::create_no_ret_ast(tok->t_span());
    this->m_index++;
    return no_ret;
  } else if (tok->t_kind()==token_kind::kw_void) {
    auto void_ast = parser::create_void_ast(tok->t_span());
    this->m_index++;
    return void_ast;
  } else if (tok->t_kind()==token_kind::kw_true || tok->t_kind()==token_kind::kw_false) {
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

std::unique_ptr<ast> parser::parse_group(bool is_required) {
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

std::unique_ptr<ast> parser::parse_expr(bool is_required) {
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

} // namespace dal::core