set(sources
        CLI.hpp
        CLI.cpp Command.cpp Command.hpp Argument.cpp Argument.hpp Context.cpp Context.hpp)

add_library(cli STATIC ${sources})
target_include_directories(cli PRIVATE ${CMAKE_SOURCE_DIR})
target_link_libraries(cli PRIVATE utils)