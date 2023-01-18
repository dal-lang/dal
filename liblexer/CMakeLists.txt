set(sources Token.cpp Token.hpp Cursor.cpp Cursor.hpp Lexer.cpp Lexer.hpp)
add_library(lexer STATIC ${sources})
target_link_libraries(lexer PRIVATE utils span)
target_include_directories(lexer PUBLIC ${CMAKE_SOURCE_DIR})