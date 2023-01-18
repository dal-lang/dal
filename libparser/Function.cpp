/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Parser.hpp"

namespace dal {

    std::shared_ptr<NodeFnDef> Parser::parse_fn_def(bool is_required) {
        auto proto = this->parse_fn_proto(is_required);
        if (!proto) {
            return nullptr;
        }

        auto block = this->parse_block(true);

        auto *fn_def = new NodeFnDef();
        fn_def->m_span = proto->m_span;
        fn_def->merge_span(block);
        fn_def->m_proto = std::move(proto);
        fn_def->m_block = std::move(block);
        return std::shared_ptr<NodeFnDef>(fn_def);
    }

    std::shared_ptr<NodeFnProto> Parser::parse_fn_proto(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];

        bool is_pub = false;
        if (token->kind() == TOK_PUB) {
            is_pub = true;
            this->m_index++;
            Token *fn_tok = &this->m_tokens[this->m_index];
            this->m_index++;
            this->expect_token(fn_tok, TOK_FN);
        } else if (token->kind() == TOK_FN) {
            this->m_index++;
        } else {
            if (is_required) {
                this->error(token, "Expected function definition");
            }
            return nullptr;
        }

        Token *fn_name = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(fn_name, TOK_IDENT);

        std::string fn_ident = this->token_value(fn_name);
        bool is_variadic = false;
        auto params = this->parse_param_decls(&is_variadic);
        std::shared_ptr<NodeType> ret_type;

        Token *maybe_arrow = &this->m_tokens[this->m_index];
        if (maybe_arrow->kind() == TOK_ARROW) {
            this->m_index++;
            ret_type = this->parse_type();
        } else {
            auto void_ty = new NodeTypePrimitive();
            void_ty->m_name = "void";
            void_ty->m_span = maybe_arrow->span();
            ret_type = std::shared_ptr<NodeTypePrimitive>(void_ty);
        }

        auto *fn_proto = new NodeFnProto();
        fn_proto->m_span = token->span();
        fn_proto->merge_span(ret_type);
        fn_proto->m_is_public = is_pub;
        fn_proto->m_name = fn_ident;
        fn_proto->m_params = std::move(params);
        fn_proto->m_is_variadic = is_variadic;
        fn_proto->m_return_type = std::move(ret_type);
        this->m_attributes.clear();
        return std::shared_ptr<NodeFnProto>(fn_proto);
    }

    std::vector<std::shared_ptr<NodeParamDecl>> Parser::parse_param_decls(bool *is_variadic) {
        Token *lparen = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(lparen, TOK_LPAREN);

        Token *maybe_rparen = &this->m_tokens[this->m_index];
        if (maybe_rparen->kind() == TOK_RPAREN) {
            this->m_index++;
            return {};
        }

        std::vector<std::shared_ptr<NodeParamDecl>> params;
        for (;;) {
            auto param = this->parse_param_decl();
            bool expect_end = false;
            if (param) {
                params.push_back(std::move(param));
            } else {
                expect_end = true;
                *is_variadic = true;
            }

            Token *token = &this->m_tokens[this->m_index];
            this->m_index++;
            if (token->kind() == TOK_RPAREN) {
                return params;
            } else if (expect_end) {
                this->error(token, "Expected end of parameter list");
            } else {
                this->expect_token(token, TOK_COMMA);
            }
        }
    }

    std::shared_ptr<NodeParamDecl> Parser::parse_param_decl() {
        Token *token = &this->m_tokens[this->m_index];
        this->m_index++;

        if (token->kind() == TOK_IDENT) {
            std::string ident = this->token_value(token);

            Token *colon = &this->m_tokens[this->m_index];
            this->m_index++;
            this->expect_token(colon, TOK_COLON);

            auto type = this->parse_type();

            auto *param = new NodeParamDecl();
            param->m_span = token->span();
            param->merge_span(type);
            param->m_name = ident;
            param->m_type = std::move(type);
            return std::shared_ptr<NodeParamDecl>(param);
        } else if (token->kind() == TOK_ELLIPSIS) {
            return nullptr;
        } else {
            this->error(token, "Expected parameter declaration");
        }
    }

    std::shared_ptr<NodeFnDecl> Parser::parse_fn_decl() {
        auto proto = this->parse_fn_proto(true);
        auto *fn_decl = new NodeFnDecl();
        fn_decl->m_span = proto->m_span;
        fn_decl->m_proto = std::move(proto);
        return std::shared_ptr<NodeFnDecl>(fn_decl);
    }

    std::vector<std::shared_ptr<Ast>> Parser::parse_call_params() {
        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() == TOK_RPAREN) {
            this->m_index++;
            return {};
        }

        std::vector<std::shared_ptr<Ast>> params;
        for (;;) {
            auto expr = this->parse_expr(true);
            params.push_back(std::move(expr));

            Token *maybe_rparen = &this->m_tokens[this->m_index];
            this->m_index++;
            if (maybe_rparen->kind() == TOK_RPAREN) {
                return params;
            } else {
                this->expect_token(maybe_rparen, TOK_COMMA);
            }
        }
    }

} // dal