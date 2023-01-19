set(dal_src utils/fmt.cc main.cc utils/os.cc utils/utils.cc cli/cli.cc cli/arg.cc cli/cmd.cc cli/ctx.cc)

add_executable(dal ${dal_src})
target_include_directories(dal PUBLIC ${CMAKE_CURRENT_SOURCE_DIR} ${CMAKE_BINARY_DIR})