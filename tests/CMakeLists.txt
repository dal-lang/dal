# GTest
# set policy CMP0135
#cmake_policy(SET CMP0135 NEW)
include(FetchContent)
FetchContent_Declare(
        googletest
        URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

enable_testing()
include(GoogleTest)

include_directories(${gtest_SOURCE_DIR}/include ${gtest_SOURCE_DIR} ${CMAKE_SOURCE_DIR}/include)

set(test_src test_tokenizer.cc test_parser.cc ${DAL_SRC})

add_executable(dal_test ${test_src})
set_target_properties(dal_test PROPERTIES CXX_STANDARD 17)
target_link_libraries(dal_test gtest gtest_main ${LLVM_LIBRARIES})
gtest_discover_tests(dal_test)