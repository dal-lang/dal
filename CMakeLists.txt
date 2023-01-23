cmake_minimum_required(VERSION 3.20)

set(DAL_CMAKE_DIR
        "${CMAKE_CURRENT_LIST_DIR}/cmake"
        CACHE PATH "Path to DAL CMake scripts")
list(APPEND CMAKE_MODULE_PATH "${DAL_CMAKE_DIR}")

include(FindCompiler)
set(CMAKE_CXX_STANDARD 17)

# Set default build type to Release
if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Release")
endif ()

# Set cmake policies
include(DalPolicy)
dal_policy()

# Project name and version should be set after cmake_policy CMP0048
set(PROJECT_VERSION 0.1.0)
project(
        dal
        VERSION ${PROJECT_VERSION}
        LANGUAGES C CXX)

# STD library directory
set(DAL_STD_DIR "${CMAKE_SOURCE_DIR}/std" CACHE PATH "Path to the standard library")

# License
file(READ ${CMAKE_SOURCE_DIR}/LICENSE LICENSE_TEXT HEX)
string(REGEX MATCHALL ".." LICENSE_TEXT "${LICENSE_TEXT}")
string(REGEX REPLACE ";" ",\n\t0x" LICENSE_TEXT "${LICENSE_TEXT}")
set(LICENSE_TEXT "0x${LICENSE_TEXT}")

# Configure config.h
configure_file("${CMAKE_SOURCE_DIR}/cmake/templates/config.h.in"
        "${CMAKE_BINARY_DIR}/config.h")

# Find llvm
include(FindLLVM)

# Export compile-cli_command.json for clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Add subdirectories
add_subdirectory(src)

# Add tests, can be disabled with -DDAL_BUILD_TESTS=OFF
option(DAL_BUILD_TESTS "Build tests" ON)
if (DAL_BUILD_TESTS)
    add_subdirectory(tests)
endif ()