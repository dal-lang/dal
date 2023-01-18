/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Parser.hpp"

namespace dal {

    std::shared_ptr<NodeStrLit> Parser::parse_str_lit(Token *token) {
        bool escaped = false;
        bool first = true;
        Span span = token->span();
        std::string str;
        for (int i = span.start_pos(); i < span.end_pos() - 1; i++) {
            char c = this->m_source[i];

            if (first) {
                first = false;
            } else {
                if (escaped) {
                    switch (c) {
                        case '\\':
                            str += '\\';
                            break;
                        case 'r':
                            str += '\r';
                            break;
                        case 'n':
                            str += '\n';
                            break;
                        case 't':
                            str += '\t';
                            break;
                        case '"':
                            str += '"';
                            break;
                        default:
                            break;
                    }
                    escaped = false;
                } else if (c == '\\') {
                    escaped = true;
                } else {
                    str += c;
                }
            }
        }

        auto node = new NodeStrLit();
        node->m_span = span;
        node->m_value = str;
        return std::shared_ptr<NodeStrLit>(node);
    }

    std::shared_ptr<NodeNumLit> Parser::parse_num_lit(Token *token) {
        auto *node = new NodeNumLit();
        node->m_span = token->span();
        node->m_value = this->token_value(token);
        return std::shared_ptr<NodeNumLit>(node);
    }

} // dal