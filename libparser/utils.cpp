/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Parser.hpp"
#include <libutils/Fmt.hpp>
#include <libspan/Error.hpp>

namespace dal {

    void Parser::expect_token(Token *token, TokenKind kind, std::string msg) {
        if (token->kind() != kind) {
            if (msg.empty()) {
                Token tok(kind, token->span());
                msg = Fmt::format("expected %s, got %s", tok.kindString(), token->kindString());
            }
            Error err(msg, token->span());
            err.panic(this->m_source);
        }
    }

    std::string Parser::token_value(Token *token) {
        return this->m_source.substr(token->span().start_pos(), token->span().len());
    }

    void Parser::error(Token *token, const std::string &msg) {
        Error err(msg, token->span());
        err.panic(this->m_source);
    }

}
