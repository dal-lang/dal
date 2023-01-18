/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Parser.hpp"

namespace dal {
    std::shared_ptr<NodeRoot> Parser::parse() {
        NodeRoot root;

        for (;;) {
            Token *attr = &this->m_tokens[this->m_index];
            this->parse_attributes();

            auto fn_def = this->parse_fn_def(false);
            if (fn_def) {
                root.merge_span(fn_def);
                root.m_children.push_back(std::move(fn_def));
                continue;
            }

            auto extern_block = this->parse_extern_block(false);
            if (extern_block) {
                root.merge_span(extern_block);
                root.m_children.push_back(std::move(extern_block));
                continue;
            }

            auto import = this->parse_import();
            if (import) {
                root.merge_span(import);
                root.m_children.push_back(std::move(import));
                continue;
            }

            if (!this->m_attributes.empty()) {
                this->error(attr, "Unexpected attribute");
            }

            break;
        }

        return std::make_shared<NodeRoot>(std::move(root));
    }

    void Parser::parse_attributes() {
        for (;;) {
            Token *token = &this->m_tokens[this->m_index];
            if (token->kind() == TOK_AT) {
                this->parse_attribute();
            } else {
                return;
            }
        }
    }

    void Parser::parse_attribute() {
        Token *token = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(token, TOK_AT);

        Token *name = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(name, TOK_IDENT);

        Token *lparen = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(lparen, TOK_LPAREN);

        Token *value = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(value, TOK_STRING);

        Token *rparen = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(rparen, TOK_RPAREN);

        NodeAttribute attr;
        auto rparen_span = rparen->span();
        attr.m_span = token->span().merge(rparen_span);
        attr.m_name = this->token_value(name);
        attr.m_param = this->token_value(value);
        this->m_attributes.push_back(std::make_shared<NodeAttribute>(attr));
    }

    std::shared_ptr<NodeType> Parser::parse_type() {
        Token *token = &this->m_tokens[this->m_index];
        this->m_index++;

        std::shared_ptr<NodeType> type;

        if (token->kind() == TOK_BANG) {
            // no return type
            NodeTypePrimitive no_return;
            no_return.m_span = token->span();
            no_return.m_name = "!";
            type = std::make_shared<NodeTypePrimitive>(no_return);
        } else if (token->kind() == TOK_VOID) {
            // void type
            NodeTypePrimitive void_type;
            void_type.m_span = token->span();
            void_type.m_name = "void";
            type = std::make_shared<NodeTypePrimitive>(void_type);
        } else if (token->kind() == TOK_IDENT) {
            // primitive type
            NodeTypePrimitive primitive;
            primitive.m_span = token->span();
            primitive.m_name = this->token_value(token);
            type = std::make_shared<NodeTypePrimitive>(primitive);
        } else if (token->kind() == TOK_STAR) {
            // pointer type: *const T | *mut T
            Token *const_or_mut = &this->m_tokens[this->m_index];
            this->m_index++;
            bool is_const;
            if (const_or_mut->kind() == TOK_CONST) {
                is_const = true;
            } else if (const_or_mut->kind() == TOK_MUT) {
                is_const = false;
            } else {
                this->error(const_or_mut, "Expected const or mut");
            }
            auto inner_type = this->parse_type();
            NodeTypePointer pointer_type;
            pointer_type.m_span = token->span().merge(inner_type->m_span);
            pointer_type.m_elem_type = std::move(inner_type);
            pointer_type.m_is_mut = !is_const;
            type = std::make_shared<NodeTypePointer>(pointer_type);
        } else if (token->kind() == TOK_LBRACKET) {
            // array type: [T; N]
            auto inner_type = this->parse_type();

            Token *semicolon = &this->m_tokens[this->m_index];
            this->m_index++;
            this->expect_token(semicolon, TOK_SEMICOLON);

            Token *size = &this->m_tokens[this->m_index];
            this->m_index++;
            this->expect_token(size, TOK_INT);

            auto node_size = this->parse_num_lit(size);

            Token *rbracket = &this->m_tokens[this->m_index];
            this->m_index++;
            this->expect_token(rbracket, TOK_RBRACKET);
            auto last_span = rbracket->span();

            NodeTypeArray array_type;
            array_type.m_elem_type = std::move(inner_type);
            array_type.m_size = std::move(node_size);
            array_type.m_span = token->span().merge(last_span);
            type = std::make_shared<NodeTypeArray>(array_type);
        } else {
            this->error(token, "Expected type");
        }

        return std::move(type);
    }

    std::shared_ptr<NodeImport> Parser::parse_import() {
        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_IMPORT) {
            return nullptr;
        }
        this->m_index++;

        Token *path = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(path, TOK_STRING);
        auto node_path = this->parse_str_lit(path);

        NodeImport import;
        import.m_span = token->span();
        import.merge_span(node_path);
        import.m_path = std::move(node_path);
        return std::make_shared<NodeImport>(import);
    }

    std::shared_ptr<Ast> Parser::parse_postfix_expr(bool is_required) {
        auto primary_expr = this->parse_primary_expr(is_required);
        if (!primary_expr)
            return nullptr;

        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() == TOK_LPAREN) {
            // function call
            this->m_index++;

            NodeFnCallExpr fn_call;
            fn_call.m_span = primary_expr->m_span;
            if (primary_expr->ast_kind != AstKindIdent)
                this->error(token, "Expected identifier");

            auto ident = std::static_pointer_cast<NodeIdent>(primary_expr);
            fn_call.m_name = ident;

            fn_call.m_params = this->parse_call_params();
            if (!fn_call.m_params.empty())
                fn_call.merge_span(fn_call.m_params.back());

            return std::make_shared<NodeFnCallExpr>(fn_call);
        } else if (token->kind() == TOK_LBRACKET) {
            // array access
            this->m_index++;

            NodeArrayAccessExpr array_access;
            array_access.m_span = primary_expr->m_span;
            array_access.m_array = std::move(primary_expr);
            array_access.m_index = this->parse_expr(true);

            Token *rbracket = &this->m_tokens[this->m_index];
            this->m_index++;
            this->expect_token(rbracket, TOK_RBRACKET);
            auto last_span = rbracket->span();
            array_access.m_span = array_access.m_span.merge(last_span);

            return std::make_shared<NodeArrayAccessExpr>(array_access);
        } else {
            return primary_expr;
        }
    }

    std::shared_ptr<Ast> Parser::parse_primary_expr(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];

        if (token->kind() == TOK_INT) {
            // number literal
            auto num_lit = this->parse_num_lit(token);
            this->m_index++;
            return std::move(num_lit);
        } else if (token->kind() == TOK_STRING) {
            // string literal
            auto str_lit = this->parse_str_lit(token);
            this->m_index++;
            return std::move(str_lit);
        } else if (token->kind() == TOK_BANG) {
            // noreturn
            NodeNoReturn no_return;
            no_return.m_span = token->span();
            this->m_index++;
            return std::make_shared<NodeNoReturn>(no_return);
        } else if (token->kind() == TOK_VOID) {
            // void
            NodeVoid void_type;
            void_type.m_span = token->span();
            this->m_index++;
            return std::make_shared<NodeVoid>(void_type);
        } else if (token->kind() == TOK_TRUE || token->kind() == TOK_FALSE) {
            // boolean literal
            NodeBoolLit bool_lit;
            bool_lit.m_span = token->span();
            bool_lit.m_value = token->kind() == TOK_TRUE;
            this->m_index++;
            return std::make_shared<NodeBoolLit>(bool_lit);
        } else if (token->kind() == TOK_IDENT) {
            // identifier
            NodeIdent ident;
            ident.m_span = token->span();
            ident.m_value = this->token_value(token);
            this->m_index++;
            return std::make_shared<NodeIdent>(ident);
        }

        // group: (expr)
        auto group = this->parse_group_expr(false);
        if (group)
            return group;

        if (!is_required)
            return nullptr;

        this->error(token, "Expected expression");
    }

    std::shared_ptr<Ast> Parser::parse_group_expr(bool is_required) {
        Token *token = &this->m_tokens[this->m_index];
        if (token->kind() != TOK_LPAREN) {
            if (is_required)
                this->error(token, "Expected expression");
            return nullptr;
        }
        this->m_index++;

        auto expr = this->parse_expr(true);

        Token *rparen = &this->m_tokens[this->m_index];
        this->m_index++;
        this->expect_token(rparen, TOK_RPAREN);

        return expr;
    }

} // dal