include_directories(../include ${CMAKE_BINARY_DIR})

# ./fmt
add_library(fmt fmt_impl.cc)

# ./cli
set(cli_srcs cli/app.cc cli/arg.cc cli/cmd.cc cli/ctx.cc)
add_library(cli ${cli_srcs})
target_link_libraries(cli PUBLIC fmt)

# ./core
set(core_srcs core/ast.cc core/error.cc core/os.cc core/parser.cc core/span.cc core/token.cc core/tokenizer.cc core/function_parser.cc core/block_parser.cc core/expr_parser.cc core/codegen.cc core/llvm.cc core/table_entry.cc)
add_library(core ${core_srcs})
target_link_libraries(core PRIVATE fmt ${llvm_libs})

set(dal_src main.cc)
add_executable(dal ${dal_src})
target_link_libraries(dal cli core fmt)