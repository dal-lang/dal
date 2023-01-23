/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/parser.hh>

namespace dal::core {

std::unique_ptr<block_ast> parser::parse_block(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];

  if (tok->t_kind()!=token_kind::punct_lbrace) {
    if (is_required) {
      this->error(*tok, "Expected '{'");
    }
    return nullptr;
  }
  this->m_index++;

  auto block = std::make_unique<block_ast>();
  block->set_span(tok->t_span());

  for (;;) {
    std::unique_ptr<ast> stmt = this->parse_var_decl(false);
    if (!stmt)
      stmt = this->parse_if_or_block(false);
    if (!stmt)
      stmt = this->parse_return_or_assign(false);
    if (!stmt)
      stmt = parser::create_void_ast(tok->t_span());

    block->add_child(std::move(stmt));

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

std::unique_ptr<ast> parser::parse_if_or_block(bool is_required) {
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

std::unique_ptr<if_ast> parser::parse_if(bool is_required) {
  auto tok = &this->m_tokens[this->m_index];
  if (tok->t_kind()!=token_kind::kw_if) {
    if (is_required) {
      this->error(*tok, "Expected if expression");
    }
    return nullptr;
  }
  this->m_index++;

  auto if_node = std::make_unique<if_ast>();
  if_node->set_span(tok->t_span());

  auto cond = this->parse_expr(true);
  if_node->set_cond(std::move(cond));
  auto block = this->parse_block(true);
  if_node->set_then(std::move(block));
  auto else_node = this->parse_else(false);
  if (else_node)
    if_node->set_else(std::move(else_node));

  return if_node;
}

std::unique_ptr<ast> parser::parse_else(bool is_required) {
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

} // namespace dal::core