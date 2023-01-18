set(sources Ast.cpp Ast.hpp)
add_library(ast STATIC ${sources})
target_link_libraries(ast PRIVATE span)
target_include_directories(ast PRIVATE ${CMAKE_SOURCE_DIR})