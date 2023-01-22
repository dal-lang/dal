//
// Created by ademrx0 on 19/01/23.
//

#include <gtest/gtest.h>
#include <fmt/core.hh>

TEST(fmt, format) {
  EXPECT_EQ(fmt::format("Hello, {}!", "world"), "Hello, world!");
  EXPECT_EQ(fmt::format("The answer is {}", 42), "The answer is 42");
  EXPECT_EQ(fmt::format("The value is {}", true), "The value is true");
}

TEST(fmt, printer) {
  testing::internal::CaptureStdout();

  fmt::println("Hello, {}!", "world");

  EXPECT_EQ(testing::internal::GetCapturedStdout(), "Hello, world!\n");

  std::ostringstream oss;
  fmt::print(oss, "The answer is {}", 42);
  EXPECT_EQ(oss.str(), "The answer is 42");
}
