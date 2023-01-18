/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Parser.hpp"

namespace dal {

    std::shared_ptr<Ast> Parser::parse_non_block_expr(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];

        auto ret = this->parse_return_expr(false);
        if (ret)
            return ret;

        auto assign = this->parse_assign_expr(false);
        if (assign)
            return assign;

        if (is_required)
            this->error(token, "Expected expression");

        return nullptr;
    }

    std::shared_ptr<NodeVarDecl> Parser::parse_var_decl(bool is_required) {
        Token *let = &this->m_tokens[this->m_index];
        if (let->kind() != TOK_LET) {
            if (is_required) {
                this->error(let, "Expected variable declaration");
            }
            return nullptr;
        }
        this->m_index++;

        NodeVarDecl varDecl;

        Token *ident;
        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() == TOK_MUT) {
            varDecl.m_is_const = false;
            this->m_index++;
            ident = &this->m_tokens[this->m_index];
            this->expect_token(ident, TOK_IDENT);
        } else if (token->kind() == TOK_IDENT) {
            varDecl.m_is_const = true;
            ident = token;
        } else {
            this->error(token, "Expected identifier");
        }
        this->m_index++;

        varDecl.m_name = this->token_value(ident);
        varDecl.m_span = ident->span();

        Token *maybe_colon = &this->m_tokens[this->m_index];
        this->m_index++;
        if (maybe_colon->kind() == TOK_ASSIGN) {
            varDecl.m_value = this->parse_expr(true);
            varDecl.merge_span(varDecl.m_value);
            return std::make_shared<NodeVarDecl>(varDecl);
        } else if (maybe_colon->kind() == TOK_COLON) {
            varDecl.m_type = this->parse_type();
            varDecl.merge_span(varDecl.m_type);

            Token *maybe_assign = &this->m_tokens[this->m_index];
            if (maybe_assign->kind() == TOK_ASSIGN) {
                this->m_index++;
                varDecl.m_value = this->parse_expr(true);
                varDecl.merge_span(varDecl.m_value);
            }
            return std::make_shared<NodeVarDecl>(varDecl);
        } else {
            this->error(maybe_colon, "Expected : or =");
        }
    }

    std::shared_ptr<NodeReturnExpr> Parser::parse_return_expr(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_RETURN) {
            if (is_required) {
                this->error(token, "Expected return expression");
            }
            return nullptr;
        }
        this->m_index++;

        NodeReturnExpr ret;
        ret.m_span = token->span();
        ret.m_value = this->parse_expr(false);
        if (ret.m_value)
            ret.merge_span(ret.m_value);

        return std::make_shared<NodeReturnExpr>(std::move(ret));
    }

    std::shared_ptr<Ast> Parser::parse_expr(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];

        auto block = this->parse_block_expr(false);
        if (block)
            return block;

        auto non_block = this->parse_non_block_expr(false);
        if (non_block)
            return non_block;

        if (is_required)
            this->error(token, "Expected expression");

        return nullptr;
    }

    std::shared_ptr<Ast> Parser::parse_assign_expr(bool is_required) {
        auto lhs = this->parse_log_or_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_ASSIGN)
            return lhs;
        this->m_index++;

        auto rhs = this->parse_log_or_expr(true);

        NodeBinOpExpr binOp(BinOpAssign);
        binOp.m_span = lhs->m_span;
        binOp.merge_span(rhs);
        binOp.m_left = lhs;
        binOp.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binOp));
    }

} // dal