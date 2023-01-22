include_directories(../include ${CMAKE_BINARY_DIR})

# ./cli
set(cli_srcs cli/app.cc cli/arg.cc cli/cmd.cc cli/ctx.cc)
add_library(cli ${cli_srcs} fmt_impl.cc)

# ./core
set(core_srcs core/ast.cc core/error.cc core/os.cc core/parser.cc core/span.cc core/token.cc core/tokenizer.cc core/function_parser.cc core/block_parser.cc core/expr_parser.cc)
add_library(core ${core_srcs} fmt_impl.cc)

set(dal_src main.cc)
add_executable(dal ${dal_src})
target_link_libraries(dal cli core)