/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Parser.hpp"

namespace dal {

    std::shared_ptr<Ast> Parser::parse_log_or_expr(bool is_required) {
        auto lhs = this->parse_log_and_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_LOGIC_OR)
            return lhs;
        this->m_index++;

        auto rhs = this->parse_log_and_expr(true);

        NodeBinOpExpr binop(BinOpLogicalOr);
        binop.m_span = lhs->m_span;
        binop.m_span.merge(rhs->m_span);
        binop.m_left = lhs;
        binop.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binop));
    }

    std::shared_ptr<Ast> Parser::parse_log_and_expr(bool is_required) {
        auto lhs = this->parse_cmp_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_LOGIC_AND)
            return lhs;
        this->m_index++;

        auto rhs = this->parse_cmp_expr(true);

        NodeBinOpExpr binop(BinOpLogicalAnd);
        binop.m_span = lhs->m_span;
        binop.m_span.merge(rhs->m_span);
        binop.m_left = lhs;
        binop.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binop));
    }

    std::shared_ptr<Ast> Parser::parse_cmp_expr(bool is_required) {
        auto lhs = this->parse_bit_or_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        BinOp op;
        switch (token->kind()) {
            case TOK_EQ:
                op = BinOpCmpEq;
                break;
            case TOK_NE:
                op = BinOpCmpNeq;
                break;
            case TOK_LT:
                op = BinOpCmpLt;
                break;
            case TOK_GT:
                op = BinOpCmpGt;
                break;
            case TOK_LE:
                op = BinOpCmpLte;
                break;
            case TOK_GE:
                op = BinOpCmpGte;
                break;
            default:
                return lhs;
        }
        this->m_index++;

        auto rhs = this->parse_bit_or_expr(true);

        NodeBinOpExpr binop(op);
        binop.m_span = lhs->m_span;
        binop.m_span.merge(rhs->m_span);
        binop.m_left = lhs;
        binop.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binop));
    }

    std::shared_ptr<Ast> Parser::parse_bit_or_expr(bool is_required) {
        auto lhs = this->parse_bit_xor_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_PIPE)
            return lhs;
        this->m_index++;

        auto rhs = this->parse_bit_xor_expr(true);

        NodeBinOpExpr binop(BinOpBitOr);
        binop.m_span = lhs->m_span;
        binop.m_span.merge(rhs->m_span);
        binop.m_left = lhs;
        binop.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binop));
    }

    std::shared_ptr<Ast> Parser::parse_bit_xor_expr(bool is_required) {
        auto lhs = this->parse_bit_and_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_CARET)
            return lhs;
        this->m_index++;

        auto rhs = this->parse_bit_and_expr(true);

        NodeBinOpExpr binop(BinOpBitXor);
        binop.m_span = lhs->m_span;
        binop.m_span.merge(rhs->m_span);
        binop.m_left = lhs;
        binop.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binop));
    }

    std::shared_ptr<Ast> Parser::parse_bit_and_expr(bool is_required) {
        auto lhs = this->parse_shift_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_AND)
            return lhs;
        this->m_index++;

        auto rhs = this->parse_shift_expr(true);

        NodeBinOpExpr binop(BinOpBitAnd);
        binop.m_span = lhs->m_span;
        binop.m_span.merge(rhs->m_span);
        binop.m_left = lhs;
        binop.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binop));
    }

    std::shared_ptr<Ast> Parser::parse_shift_expr(bool is_required) {
        auto lhs = this->parse_add_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        BinOp op;
        switch (token->kind()) {
            case TOK_LSHIFT:
                op = BinOpBitShiftLeft;
                break;
            case TOK_RSHIFT:
                op = BinOpBitShiftRight;
                break;
            default:
                return lhs;
        }
        this->m_index++;

        auto rhs = this->parse_add_expr(true);

        NodeBinOpExpr binop(op);
        binop.m_span = lhs->m_span;
        binop.m_span.merge(rhs->m_span);
        binop.m_left = lhs;
        binop.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binop));
    }

    std::shared_ptr<Ast> Parser::parse_add_expr(bool is_required) {
        auto lhs = this->parse_mul_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        BinOp op;
        switch (token->kind()) {
            case TOK_PLUS:
                op = BinOpAdd;
                break;
            case TOK_MINUS:
                op = BinOpSub;
                break;
            default:
                return lhs;
        }
        this->m_index++;

        auto rhs = this->parse_mul_expr(true);

        NodeBinOpExpr binop(op);
        binop.m_span = lhs->m_span;
        binop.m_span.merge(rhs->m_span);
        binop.m_left = lhs;
        binop.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binop));
    }

    std::shared_ptr<Ast> Parser::parse_mul_expr(bool is_required) {
        auto lhs = this->parse_cast_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        BinOp op;
        switch (token->kind()) {
            case TOK_STAR:
                op = BinOpMul;
                break;
            case TOK_SLASH:
                op = BinOpDiv;
                break;
            case TOK_PERCENT:
                op = BinOpMod;
                break;
            default:
                return lhs;
        }
        this->m_index++;

        auto rhs = this->parse_cast_expr(true);

        NodeBinOpExpr binop(op);
        binop.m_span = lhs->m_span;
        binop.m_span.merge(rhs->m_span);
        binop.m_left = lhs;
        binop.m_right = rhs;
        return std::make_shared<NodeBinOpExpr>(std::move(binop));
    }

    std::shared_ptr<Ast> Parser::parse_cast_expr(bool is_required) {
        auto lhs = this->parse_unary_expr(is_required);
        if (!lhs)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_AS)
            return lhs;
        this->m_index++;

        NodeCastExpr cast;
        cast.m_span = lhs->m_span;
        cast.m_value = lhs;
        cast.m_type = this->parse_type();
        cast.merge_span(cast.m_type);
        return std::make_shared<NodeCastExpr>(std::move(cast));
    }

    std::shared_ptr<Ast> Parser::parse_unary_expr(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];
        UnOp op;
        switch (token->kind()) {
            case TOK_MINUS:
                op = UnOpNeg;
                break;
            case TOK_TILDE:
                op = UnOpBitNot;
                break;
            case TOK_BANG:
                op = UnOpLogicalNot;
                break;
            default:
                return this->parse_postfix_expr(is_required);
        }
        this->m_index++;

        auto rhs = this->parse_postfix_expr(true);

        NodeUnOpExpr unop(op);
        unop.m_span = token->span();
        unop.merge_span(rhs);
        unop.m_value = rhs;
        return std::make_shared<NodeUnOpExpr>(std::move(unop));
    }

} // dal