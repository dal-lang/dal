//
// Created by ademrx0 on 22/01/23.
//

#include <gtest/gtest.h>
#include <core/parser.hh>
#include <core/tokenizer.hh>

TEST(SimpleParsing, SimpleParsing) {
  std::string input = "pub fn add(a: u8, b: u8) -> u8 { return a + b }";
  dal::core::tokenizer tk;
  auto tokens = tk.tokenize(input);
  ASSERT_EQ(tokens.size(), 21); // input + EOF
  dal::core::parser ps(input, tokens);
  auto ast = ps.parse();
  ASSERT_EQ(ast->kind(), dal::core::ast_kind::root_node);
}