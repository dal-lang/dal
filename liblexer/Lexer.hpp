/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_LEXER_HPP
#define DAL_LEXER_HPP

#include <libspan/Error.hpp>
#include "Token.hpp"
#include "Cursor.hpp"

namespace dal {

    class Lexer {
    private:
        Cursor *m_cursor;
        std::string m_source;
        std::vector<Error> m_errors;

        Token next_token();

        TokenKind dash();

        TokenKind slash();

        TokenKind ampersand();

        TokenKind pipe();

        TokenKind bang();

        TokenKind equal();

        TokenKind less();

        TokenKind greater();

        TokenKind dot();

        TokenKind string();

        TokenKind number();

        TokenKind identifier();

    public:
        explicit Lexer(const std::string &str);

        std::vector<Token> lex();
    };

} // dal

#endif //DAL_LEXER_HPP
