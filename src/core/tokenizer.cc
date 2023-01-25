/*
 * Copyright (c) 2023 Ade M Ramdani
 *
 * This file is part of dal, which is MIT licensed.
 * See the file "LICENSE" for more information or visit
 * https://opensource.org/licenses/MIT for full license details.
 */

#include <core/tokenizer.hh>

namespace dal::core {

std::vector<token> tokenizer::tokenize(const std::string& str,
                                       const std::string& path) {
  this->m_str = str;
  this->m_path = path;
  this->m_src = std::vector<char>(str.begin(), str.end());
  std::vector<token> t_result;
  token t = this->next_token();
  while (t.t_kind() != token_kind::eof) {
    t_result.push_back(t);
    t = this->next_token();
  }
  t_result.push_back(t);
  return t_result;
}

bool is_whitespace(char c) {
  return c == ' ' || c == '\n';
}

bool is_digit(char c) {
  return c >= '0' && c <= '9';
}

bool is_alpha(char c) {
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

bool is_alphanum(char c) {
  return is_alpha(c) || is_digit(c);
}

bool is_identifier(char c) {
  return is_alphanum(c) || c == '_';
}

token tokenizer::next_token() {
  auto c = this->bump();
  if (c == 0)
    return {token_kind::eof, this->t_span()};

  if (is_whitespace(c)) {
    this->eat_while(is_whitespace);
    this->eat_span();
    if (this->is_eof())
      return {token_kind::eof, this->t_span()};
    c = this->bump();
  }

  token_kind t_kind;
  switch (c) {
    case ' ':
    case '\n':
      this->eat_while(is_whitespace);
      return this->next_token();
    case '(':
      t_kind = token_kind::punct_lparen;
      break;
    case ')':
      t_kind = token_kind::punct_rparen;
      break;
    case '{':
      t_kind = token_kind::punct_lbrace;
      break;
    case '}':
      t_kind = token_kind::punct_rbrace;
      break;
    case '[':
      t_kind = token_kind::punct_lbracket;
      break;
    case ']':
      t_kind = token_kind::punct_rbracket;
      break;
    case ',':
      t_kind = token_kind::punct_comma;
      break;
    case '.': {
      if (this->first() == '.' && this->second() == '.') {
        this->bump();
        this->bump();
        t_kind = token_kind::punct_ellipsis;
      } else {
        t_kind = token_kind::punct_dot;
      }
      break;
    }
    case ';':
      t_kind = token_kind::punct_semicolon;
      break;
    case ':':
      t_kind = token_kind::punct_colon;
      break;
    case '+':
      t_kind = token_kind::op_add;
      break;
    case '-': {
      if (this->first() == '>') {
        this->bump();
        t_kind = token_kind::punct_arrow;
      } else {
        t_kind = token_kind::op_sub;
      }
      break;
    }
    case '*':
      t_kind = token_kind::op_mul;
      break;
    case '/': {
      if (this->first() == '/') {
        this->eat_while([](char c) { return c != '\n'; });
        t_kind = token_kind::comment;
      } else {
        t_kind = token_kind::op_div;
      }
      break;
    }
    case '%':
      t_kind = token_kind::op_mod;
      break;
    case '^':
      t_kind = token_kind::op_xor;
      break;
    case '&': {
      if (this->first() == '&') {
        this->bump();
        t_kind = token_kind::op_log_and;
      } else {
        t_kind = token_kind::op_and;
      }
      break;
    }
    case '|': {
      if (this->first() == '|') {
        this->bump();
        t_kind = token_kind::op_log_or;
      } else {
        t_kind = token_kind::op_or;
      }
      break;
    }
    case '~':
      t_kind = token_kind::op_not;
      break;
    case '!': {
      if (this->first() == '=') {
        this->bump();
        t_kind = token_kind::op_neq;
      } else {
        t_kind = token_kind::op_log_not;
      }
      break;
    }
    case '=': {
      if (this->first() == '=') {
        this->bump();
        t_kind = token_kind::op_eq;
      } else {
        t_kind = token_kind::op_assign;
      }
      break;
    }
    case '<': {
      if (this->first() == '=') {
        this->bump();
        t_kind = token_kind::op_lte;
      } else if (this->first() == '<') {
        this->bump();
        t_kind = token_kind::op_shl;
      } else {
        t_kind = token_kind::op_lt;
      }
      break;
    }
    case '>': {
      if (this->first() == '=') {
        this->bump();
        t_kind = token_kind::op_gte;
      } else if (this->first() == '>') {
        this->bump();
        t_kind = token_kind::op_shr;
      } else {
        t_kind = token_kind::op_gt;
      }
      break;
    }
    case '@':
      t_kind = token_kind::punct_at;
      break;
    case '0' ... '9': {
      this->eat_while(is_digit);
      t_kind = token_kind::lit_int;
      break;
    }
    case 'a' ... 'z':
    case 'A' ... 'Z':
    case '_': {
      this->eat_while(is_identifier);
      auto raw = this->t_raw();
      if (raw == "let") {
        t_kind = token_kind::kw_let;
      } else if (raw == "const") {
        t_kind = token_kind::kw_const;
      } else if (raw == "mut") {
        t_kind = token_kind::kw_mut;
      } else if (raw == "if") {
        t_kind = token_kind::kw_if;
      } else if (raw == "else") {
        t_kind = token_kind::kw_else;
      } else if (raw == "fn") {
        t_kind = token_kind::kw_fn;
      } else if (raw == "return") {
        t_kind = token_kind::kw_return;
      } else if (raw == "pub") {
        t_kind = token_kind::kw_pub;
      } else if (raw == "true") {
        t_kind = token_kind::kw_true;
      } else if (raw == "false") {
        t_kind = token_kind::kw_false;
      } else if (raw == "extern") {
        t_kind = token_kind::kw_extern;
      } else if (raw == "import_node") {
        t_kind = token_kind::kw_import;
      } else if (raw == "as") {
        t_kind = token_kind::kw_as;
      } else if (raw == "void") {
        t_kind = token_kind::kw_void;
      } else {
        t_kind = token_kind::ident;
      }
      break;
    }
    case '"': {
      this->eat_while([](char c) { return c != '"'; });
      if (this->first() == '"') {
        this->bump();
        t_kind = token_kind::lit_string;
      } else {
        t_kind = token_kind::error;
        this->m_errors.emplace_back("unterminated string literal",
                                    this->t_span(), this->m_path, this->m_str);
      }
      break;
    }
    default:
      t_kind = token_kind::error;
      this->m_errors.emplace_back("invalid character", this->t_span(),
                                  this->m_path, this->m_str);
      break;
  }

  return {t_kind, this->eat_span()};
}

bool tokenizer::is_eof() {
  return this->m_src.empty();
}

char tokenizer::bump() {
  if (this->is_eof()) {
    return 0;
  }

  char c = this->m_src[0];
  this->m_src.erase(this->m_src.begin());
  if (c == '\n') {
    this->m_span.inc_end_line();
    this->m_span.set_end_col(1);
  } else {
    this->m_span.inc_end_col();
  }
  this->m_span.inc_end_pos();
  return c;
}

char tokenizer::first() {
  if (this->is_eof()) {
    return 0;
  }
  return this->m_src[0];
}

char tokenizer::second() {
  if (this->is_eof() || this->m_src.size() < 2) {
    return 0;
  }
  return this->m_src[1];
}

void tokenizer::eat_while(const std::function<bool(char)>& pred) {
  while (!this->is_eof() && pred(this->first())) {
    this->bump();
  }
}

span tokenizer::t_span() {
  return this->m_span;
}

span tokenizer::eat_span() {
  auto old = this->t_span();
  this->m_span.update();
  return old;
}

std::string tokenizer::t_raw() {
  return this->m_str.substr(this->t_span().start_pos(), this->t_span().len());
}

bool tokenizer::has_error() const {
  return !this->m_errors.empty();
}

void tokenizer::print_error() {
  for (auto& e : this->m_errors) {
    e.raise(false);
  }
  exit(1);
}

}  // namespace dal::core