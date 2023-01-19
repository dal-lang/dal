/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#ifndef DAL_TOKEN_HPP
#define DAL_TOKEN_HPP

#include <string>
#include "src/utils/span.hpp"

namespace dal {

    enum TokenKind {
        // Single character tokens
        TOK_LPAREN, // (
        TOK_RPAREN, // )
        TOK_LBRACE, // {
        TOK_RBRACE, // }
        TOK_LBRACKET, // [
        TOK_RBRACKET, // ]
        TOK_SEMICOLON, // ;
        TOK_COMMA, // ,
        TOK_DOT, // .
        TOK_MINUS, // -
        TOK_PLUS, // +
        TOK_SLASH, // /
        TOK_STAR, // *
        TOK_PERCENT, // %
        TOK_TILDE, // ~
        TOK_BANG, // !
        TOK_ASSIGN, // =
        TOK_CARET, // ^
        TOK_PIPE, // |
        TOK_AND, // &
        TOK_LT, // <
        TOK_GT, // >
        TOK_COLON, // :
        TOK_BACKSLASH, // '\'
        TOK_AT, // @

        // Two character tokens
        TOK_EQ, // ==
        TOK_NE, // !=
        TOK_LE, // <=
        TOK_GE, // >=
        TOK_LSHIFT, // <<
        TOK_RSHIFT, // >>
        TOK_LOGIC_AND, // &&
        TOK_LOGIC_OR, // ||
        TOK_ARROW, // ->
        TOK_ELLIPSIS, // ...

        // Literals
        TOK_INT, // Integer literal
        TOK_CHAR, // Character literal
        TOK_STRING, // String literal
        TOK_IDENT, // Identifier
        TOK_COMMENT, // Comment

        // Keywords
        TOK_AS, // as
        TOK_CONST, // const
        TOK_MUT, // mut
        TOK_FN, // fn
        TOK_IF, // if
        TOK_ELSE, // else
        TOK_RETURN, // return
        TOK_IMPORT, // import
        TOK_TRUE, // true
        TOK_FALSE, // false
        TOK_PUB, // pub
        TOK_EXTERN, // extern
        TOK_VOID, // void
        TOK_LET, // let

        // Special tokens
        TOK_EOF, // End of file
        TOK_UNKNOWN, // Unknown token
    };

    class Token {
    private:
        TokenKind token_kind = TOK_EOF;
        Span token_span{};
    public:
        Token() = default;

        Token(TokenKind token_kind, Span token_span);

        void setKind(TokenKind kind);

        void setSpan(Span span);

        [[nodiscard]] TokenKind kind() const;

        [[nodiscard]] Span span() const;

        [[nodiscard]] std::string kindString() const;

        [[nodiscard]] std::string toString(const std::string &source) const;
    };

} // dal

#endif //DAL_TOKEN_HPP
