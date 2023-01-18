/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include "Lexer.hpp"

static bool is_whitespace(char c) {
    return c == ' ' || c == '\n';
}

static bool not_line_break(char c) {
    return c != '\n';
}

static bool is_digit(char c) {
    return c >= '0' && c <= '9';
}

static bool is_alpha(char c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

static bool is_alnum(char c) {
    return is_alpha(c) || is_digit(c);
}

static bool is_identifier(char c) {
    return is_alnum(c) || c == '_';
}

namespace dal {

    Lexer::Lexer(const std::string &str) {
        this->m_cursor = new Cursor(str);
        this->m_source = str;
    }

    Token Lexer::next_token() {
        auto c = this->m_cursor->bump();
        if (c == 0) {
            return {TOK_EOF, this->m_cursor->span()};
        }

        if (is_whitespace(c)) {
            this->m_cursor->eat_while(is_whitespace);
            this->m_cursor->eat_span();
            if (this->m_cursor->is_eof())
                return {TOK_EOF, this->m_cursor->span()};
            c = this->m_cursor->bump();
        }

        TokenKind kind;
        switch (c) {
            case ' ':
            case '\n':
                this->m_cursor->eat_while(is_whitespace);
                return this->next_token();
            case '(':
                kind = TOK_LPAREN;
                break;
            case ')':
                kind = TOK_RPAREN;
                break;
            case '{':
                kind = TOK_LBRACE;
                break;
            case '}':
                kind = TOK_RBRACE;
                break;
            case '[':
                kind = TOK_LBRACKET;
                break;
            case ']':
                kind = TOK_RBRACKET;
                break;
            case ',':
                kind = TOK_COMMA;
                break;
            case '+':
                kind = TOK_PLUS;
                break;
            case '-':
                kind = this->dash();
                break;
            case '*':
                kind = TOK_STAR;
                break;
            case '/':
                kind = this->slash();
                break;
            case '%':
                kind = TOK_PERCENT;
                break;
            case '^':
                kind = TOK_CARET;
                break;
            case '&':
                kind = this->ampersand();
                break;
            case '|':
                kind = this->pipe();
                break;
            case '~':
                kind = TOK_TILDE;
                break;
            case '!':
                kind = this->bang();
                break;
            case '=':
                kind = this->equal();
                break;
            case '<':
                kind = this->less();
                break;
            case '>':
                kind = this->greater();
                break;
            case ':':
                kind = TOK_COLON;
                break;
            case ';':
                kind = TOK_SEMICOLON;
                break;
            case '.':
                kind = this->dot();
                break;
            case '"':
                kind = this->string();
                break;
            case '@':
                kind = TOK_AT;
                break;
            case '_':
            case 'a' ... 'z':
            case 'A' ... 'Z':
                kind = this->identifier();
                break;
            case '0' ... '9':
                kind = this->number();
                break;
            default:
                Error error("Unexpected character", this->m_cursor->span());
                this->m_errors.push_back(error);
                kind = TOK_UNKNOWN;
                break;
        }

        Token token(kind, this->m_cursor->eat_span());
        return token;
    }

    std::vector<Token> Lexer::lex() {
        std::vector<Token> tokens;
        Token token = this->next_token();
        while (token.kind() != TOK_EOF) {
            tokens.push_back(token);
            token = this->next_token();
        }
        tokens.push_back(token);
        return tokens;
    }

    TokenKind Lexer::dash() {
        if (this->m_cursor->first() == '>') {
            this->m_cursor->bump();
            return TOK_ARROW;
        }
        return TOK_MINUS;
    }

    TokenKind Lexer::slash() {
        if (this->m_cursor->first() == '/') {
            this->m_cursor->eat_while(not_line_break);
            return TOK_COMMENT;
        }
        return TOK_SLASH;
    }

    TokenKind Lexer::ampersand() {
        if (this->m_cursor->first() == '&') {
            this->m_cursor->bump();
            return TOK_LOGIC_AND;
        }
        return TOK_AND;
    }

    TokenKind Lexer::pipe() {
        if (this->m_cursor->first() == '|') {
            this->m_cursor->bump();
            return TOK_LOGIC_OR;
        }
        return TOK_PIPE;
    }

    TokenKind Lexer::bang() {
        if (this->m_cursor->first() == '=') {
            this->m_cursor->bump();
            return TOK_NE;
        }
        return TOK_BANG;
    }

    TokenKind Lexer::equal() {
        if (this->m_cursor->first() == '=') {
            this->m_cursor->bump();
            return TOK_EQ;
        }
        return TOK_ASSIGN;
    }

    TokenKind Lexer::less() {
        if (this->m_cursor->first() == '=') {
            this->m_cursor->bump();
            return TOK_LE;
        }
        return TOK_LT;
    }

    TokenKind Lexer::greater() {
        if (this->m_cursor->first() == '=') {
            this->m_cursor->bump();
            return TOK_GE;
        }
        return TOK_GT;
    }

    TokenKind Lexer::dot() {
        if (this->m_cursor->first() == '.' && this->m_cursor->second() == '.') {
            this->m_cursor->bump();
            this->m_cursor->bump();
            return TOK_ELLIPSIS;
        }
        return TOK_DOT;
    }

    TokenKind Lexer::string() {
        this->m_cursor->eat_while([](char c) { return c != '"'; });
        if (this->m_cursor->first() == '"') {
            this->m_cursor->bump();
            return TOK_STRING;
        }
        Error error("Unterminated string", this->m_cursor->span());
        this->m_errors.push_back(error);
        return TOK_UNKNOWN;
    }

    TokenKind Lexer::number() {
        this->m_cursor->eat_while(is_digit);
        return TOK_INT;
    }

    TokenKind Lexer::identifier() {
        this->m_cursor->eat_while(is_identifier);
        auto span = this->m_cursor->span();
        auto str = this->m_source;

        auto repr = str.substr(span.start_pos(), span.len());
        if (repr == "as") {
            return TOK_AS;
        } else if (repr == "const") {
            return TOK_CONST;
        } else if (repr == "mut") {
            return TOK_MUT;
        } else if (repr == "fn") {
            return TOK_FN;
        } else if (repr == "if") {
            return TOK_IF;
        } else if (repr == "else") {
            return TOK_ELSE;
        } else if (repr == "return") {
            return TOK_RETURN;
        } else if (repr == "import") {
            return TOK_IMPORT;
        } else if (repr == "true") {
            return TOK_TRUE;
        } else if (repr == "false") {
            return TOK_FALSE;
        } else if (repr == "pub") {
            return TOK_PUB;
        } else if (repr == "extern") {
            return TOK_EXTERN;
        } else if (repr == "void") {
            return TOK_VOID;
        } else if (repr == "let") {
            return TOK_LET;
        } else {
            return TOK_IDENT;
        }
    }

} // dal