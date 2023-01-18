/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Token.hpp"

namespace dal {

    Token::Token(TokenKind token_kind, Span token_span) {
        this->token_kind = token_kind;
        this->token_span = token_span;
    }

    void Token::setKind(TokenKind kind) {
        this->token_kind = kind;
    }

    TokenKind Token::kind() const {
        return this->token_kind;
    }

    std::string Token::kindString() const {
        switch (this->kind()) {
            case TOK_LPAREN:
                return "TOK_LPAREN";
            case TOK_RPAREN:
                return "TOK_RPAREN";
            case TOK_LBRACE:
                return "TOK_LBRACE";
            case TOK_RBRACE:
                return "TOK_RBRACE";
            case TOK_LBRACKET:
                return "TOK_LBRACKET";
            case TOK_RBRACKET:
                return "TOK_RBRACKET";
            case TOK_SEMICOLON:
                return "TOK_SEMICOLON";
            case TOK_COMMA:
                return "TOK_COMMA";
            case TOK_DOT:
                return "TOK_DOT";
            case TOK_MINUS:
                return "TOK_MINUS";
            case TOK_PLUS:
                return "TOK_PLUS";
            case TOK_SLASH:
                return "TOK_SLASH";
            case TOK_STAR:
                return "TOK_STAR";
            case TOK_PERCENT:
                return "TOK_PERCENT";
            case TOK_TILDE:
                return "TOK_TILDE";
            case TOK_BANG:
                return "TOK_BANG";
            case TOK_ASSIGN:
                return "TOK_ASSIGN";
            case TOK_CARET:
                return "TOK_CARET";
            case TOK_PIPE:
                return "TOK_PIPE";
            case TOK_AND:
                return "TOK_AND";
            case TOK_LT:
                return "TOK_LT";
            case TOK_GT:
                return "TOK_GT";
            case TOK_COLON:
                return "TOK_COLON";
            case TOK_BACKSLASH:
                return "TOK_BACKSLASH";
            case TOK_AT:
                return "TOK_AT";
            case TOK_EQ:
                return "TOK_EQ";
            case TOK_NE:
                return "TOK_NE";
            case TOK_LE:
                return "TOK_LE";
            case TOK_GE:
                return "TOK_GE";
            case TOK_LSHIFT:
                return "TOK_LSHIFT";
            case TOK_RSHIFT:
                return "TOK_RSHIFT";
            case TOK_LOGIC_AND:
                return "TOK_LOGIC_AND";
            case TOK_LOGIC_OR:
                return "TOK_LOGIC_OR";
            case TOK_ARROW:
                return "TOK_ARROW";
            case TOK_ELLIPSIS:
                return "TOK_ELLIPSIS";
            case TOK_INT:
                return "TOK_INT";
            case TOK_CHAR:
                return "TOK_CHAR";
            case TOK_STRING:
                return "TOK_STRING";
            case TOK_IDENT:
                return "TOK_IDENT";
            case TOK_AS:
                return "TOK_AS";
            case TOK_CONST:
                return "TOK_CONST";
            case TOK_MUT:
                return "TOK_MUT";
            case TOK_FN:
                return "TOK_FN";
            case TOK_IF:
                return "TOK_IF";
            case TOK_ELSE:
                return "TOK_ELSE";
            case TOK_RETURN:
                return "TOK_RETURN";
            case TOK_IMPORT:
                return "TOK_IMPORT";
            case TOK_TRUE:
                return "TOK_TRUE";
            case TOK_FALSE:
                return "TOK_FALSE";
            case TOK_PUB:
                return "TOK_PUB";
            case TOK_EXTERN:
                return "TOK_EXTERN";
            case TOK_EOF:
                return "TOK_EOF";
            case TOK_COMMENT:
                return "TOK_COMMENT";
            case TOK_UNKNOWN:
                return "TOK_UNKNOWN";
            case TOK_VOID:
                return "TOK_VOID";
            case TOK_LET:
                return "TOK_LET";
        }
    }

    std::string Token::toString(const std::string &source) const {
        std::string str;
        if (this->kind() == TOK_EOF) {
            str = this->kindString();
        } else {
            str = this->kindString() + " " + source.substr(this->span().start_pos(), this->span().len());
        }
        return str;
    }

    void Token::setSpan(Span span) {
        this->token_span = span;
    }

    Span Token::span() const {
        return this->token_span;
    }

} // dal