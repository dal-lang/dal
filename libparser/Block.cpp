/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Parser.hpp"

namespace dal {

    std::shared_ptr<NodeExternBlock> Parser::parse_extern_block(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_EXTERN) {
            if (is_required) {
                this->error(token, "Expected extern block");
            }
            return nullptr;
        }
        this->m_index++;

        auto *extern_block = new NodeExternBlock();
        extern_block->m_span = token->span();
        extern_block->m_attributes = this->m_attributes;
        this->m_attributes.clear();

        Token *lbrace = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(lbrace, TOK_LBRACE);

        for (;;) {
            Token *maybe_attr = &this->m_tokens[this->m_index];
            this->parse_attributes();

            Token *maybe_rbrace = &this->m_tokens[this->m_index];
            if (maybe_rbrace->kind() == TOK_RBRACE) {
                if (!this->m_attributes.empty()) {
                    this->error(maybe_attr, "Unexpected attribute");
                }
                this->m_attributes.clear();

                this->m_index++;
                Span rbrace_span = maybe_rbrace->span();
                extern_block->m_span.merge(rbrace_span);
                return std::shared_ptr<NodeExternBlock>(extern_block);
            } else {
                auto fn_decl = this->parse_fn_decl();
                extern_block->m_children.push_back(std::move(fn_decl));
            }
        }
    }

    std::shared_ptr<NodeBlock> Parser::parse_block(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];

        if (token->kind() != TOK_LBRACE) {
            if (is_required) {
                this->error(token, "Expected block");
            }
            return nullptr;
        }
        this->m_index++;

        NodeBlock block;

        for (;;) {
            std::shared_ptr<Ast> stmt = this->parse_var_decl(false);
            if (!stmt)
                stmt = this->parse_block_expr(false);
            if (!stmt)
                stmt = this->parse_non_block_expr(false);
            if (!stmt) {
                NodeVoid void_node;
                void_node.m_span = token->span();
                stmt = std::make_shared<NodeVoid>(void_node);
            }

            block.merge_span(stmt);
            block.m_children.push_back(std::move(stmt));

            token = &this->m_tokens[this->m_index];
            if (token->kind() == TOK_RBRACE) {
                this->m_index++;
                auto tok_span = token->span();
                block.m_span.merge(tok_span);
                return std::make_shared<NodeBlock>(std::move(block));
            }
        }
    }

    std::shared_ptr<Ast> Parser::parse_block_expr(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];

        auto if_expr = this->parse_if_expr(false);
        if (if_expr)
            return std::move(if_expr);

        auto block = this->parse_block(false);
        if (block)
            return std::move(block);

        if (is_required)
            this->error(token, "Expected block expression");

        return nullptr;
    }

    std::shared_ptr<NodeIfExpr> Parser::parse_if_expr(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_IF) {
            if (is_required) {
                this->error(token, "Expected if expression");
            }
            return nullptr;
        }
        this->m_index++;

        NodeIfExpr if_expr;
        if_expr.m_span = token->span();

        if_expr.m_condition = this->parse_expr(true);
        if_expr.m_then_block = this->parse_block(true);
        if_expr.m_else_block = this->parse_else_expr(false);

        if (if_expr.m_else_block) {
            if_expr.m_span.merge(if_expr.m_else_block->m_span);
        } else {
            if_expr.m_span.merge(if_expr.m_then_block->m_span);
        }

        return std::make_shared<NodeIfExpr>(std::move(if_expr));
    }

    std::shared_ptr<Ast> Parser::parse_else_expr(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_ELSE) {
            if (is_required) {
                this->error(token, "Expected else expression");
            }
            return nullptr;
        }
        this->m_index++;

        auto if_expr = this->parse_if_expr(false);
        if (if_expr)
            return std::move(if_expr);

        return this->parse_block(true);
    }

} // dal