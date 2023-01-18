set(sources Parser.cpp Parser.hpp utils.cpp Function.cpp Block.cpp Literals.cpp Expr.cpp Binop.cpp)
add_library(parser STATIC ${sources})
target_link_libraries(parser PRIVATE ast lexer)
target_include_directories(parser PRIVATE ${CMAKE_SOURCE_DIR})