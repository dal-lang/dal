include_directories(../include ${CMAKE_BINARY_DIR})

# ./cli
file(GLOB CLI_SOURCES ./cli/*.cc)
add_library(cli ${CLI_SOURCES} fmt_impl.cc)

# ./core
file(GLOB CORE_SOURCES ./core/*.cc)
add_library(core ${CORE_SOURCES} fmt_impl.cc)

set(dal_src main.cc)
add_executable(dal ${dal_src})
target_link_libraries(dal cli)