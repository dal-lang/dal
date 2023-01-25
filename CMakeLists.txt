cmake_minimum_required(VERSION 3.20)

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Debug" CACHE STRING "Choose the type of build. Available options are: Debug Release RelWithDebInfo MinSizeRel." FORCE)
endif ()

if (NOT CMAKE_INSTALL_PREFIX)
    set(CMAKE_INSTALL_PREFIX "${CMAKE_SOURCE_DIR}/install" CACHE STRING "Choose the install directory." FORCE)
endif ()

project(dal VERSION 0.1.0 LANGUAGES C CXX)
set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake ${CMAKE_MODULE_PATH})

find_package(llvm 15)

include_directories(${LLVM_INCLUDE_DIRS})

set(DAL_SRC
        "${CMAKE_SOURCE_DIR}/src/fmt/core.cc"
        "${CMAKE_SOURCE_DIR}/src/cli/app.cc"
        "${CMAKE_SOURCE_DIR}/src/cli/arg.cc"
        "${CMAKE_SOURCE_DIR}/src/cli/cmd.cc"
        "${CMAKE_SOURCE_DIR}/src/cli/ctx.cc"
        "${CMAKE_SOURCE_DIR}/src/core/analyze.cc"
        "${CMAKE_SOURCE_DIR}/src/core/ast.cc"
        "${CMAKE_SOURCE_DIR}/src/core/codegen.cc"
        "${CMAKE_SOURCE_DIR}/src/core/error.cc"
        "${CMAKE_SOURCE_DIR}/src/core/llvm.cc"
        "${CMAKE_SOURCE_DIR}/src/core/os.cc"
        "${CMAKE_SOURCE_DIR}/src/core/parser.cc"
        "${CMAKE_SOURCE_DIR}/src/core/span.cc"
        "${CMAKE_SOURCE_DIR}/src/core/table_entry.cc"
        "${CMAKE_SOURCE_DIR}/src/core/token.cc"
        "${CMAKE_SOURCE_DIR}/src/core/tokenizer.cc"
        "${CMAKE_SOURCE_DIR}/src/core/visitor.cc")

# License
file(READ ${CMAKE_SOURCE_DIR}/LICENSE LICENSE_TEXT HEX)
string(REGEX MATCHALL ".." LICENSE_TEXT "${LICENSE_TEXT}")
string(REGEX REPLACE ";" ",\n\t0x" LICENSE_TEXT "${LICENSE_TEXT}")
set(LICENSE_TEXT "0x${LICENSE_TEXT}")

# Configure files
set(DAL_STD_DIR "${CMAKE_SOURCE_DIR}/std")
configure_file(
        "${CMAKE_SOURCE_DIR}/cmake/templates/config.h.in"
        "${CMAKE_BINARY_DIR}/config.h")

include_directories(${CMAKE_SOURCE_DIR} ${CMAKE_BINARY_DIR} ${CMAKE_SOURCE_DIR}/include)

set(EXE_CXX_FLAGS "-std=c++17")

if ("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
    set(EXE_CXX_FLAGS "${EXE_CXX_FLAGS} -Werror -Wall")
    if (NOT CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_VERSION VERSION_GREATER_EQUAL 7.0)
        set(EXE_CXX_FLAGS "${EXE_CXX_FLAGS} -Werror=implicit-fallthrough")
    endif ()
    if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU" AND CMAKE_CXX_COMPILER_VERSION VERSION_LESS_EQUAL 9.2)
        set(EXE_CXX_FLAGS "${EXE_CXX_FLAGS} -Wno-maybe-uninitialized")
    endif ()
endif ()

set(EXE_CXX_FLAGS "${EXE_CXX_FLAGS} -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -D_GNU_SOURCE -fvisibility-inlines-hidden -fno-exceptions -fno-rtti -Werror=type-limits -Wno-missing-braces -Wno-comment")

add_executable(dal "${CMAKE_SOURCE_DIR}/src/main.cc" ${DAL_SRC})
set_target_properties(dal PROPERTIES COMPILE_FLAGS "${EXE_CXX_FLAGS}")
target_link_libraries(dal LINK_PRIVATE ${LLVM_LIBRARIES})

# Enable testing by using DAL_USE_TEST=ON
option(DAL_USE_TEST "Enable testing" OFF)
if (DAL_USE_TEST)
    add_subdirectory(tests)
endif ()