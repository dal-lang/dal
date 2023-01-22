//
// Created by ademrx0 on 20/01/23.
//

#include <gtest/gtest.h>
#include <core/tokenizer.hh>

TEST(Tokenizer, Tokenize) {
  std::string source = "let x = 10";
  dal::core::tokenizer tk;
  auto tokens = tk.tokenize(source);
  ASSERT_EQ(tokens.size(), 5);
  ASSERT_EQ(tokens[0].t_kind(), dal::core::token_kind::kw_let);
  ASSERT_EQ(tokens[1].t_kind(), dal::core::token_kind::ident);
  ASSERT_EQ(tokens[2].t_kind(), dal::core::token_kind::op_assign);
  ASSERT_EQ(tokens[3].t_kind(), dal::core::token_kind::lit_int);
  ASSERT_EQ(tokens[4].t_kind(), dal::core::token_kind::eof);
}