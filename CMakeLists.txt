cmake_minimum_required(VERSION 3.20)

set(DAL_CMAKE_DIR
        "${CMAKE_CURRENT_LIST_DIR}/cmake"
        CACHE PATH "Path to DAL CMake scripts")
list(APPEND CMAKE_MODULE_PATH "${DAL_CMAKE_DIR}")

# Set the build type if non was specified.
if (NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
    if (EXISTS "${CMAKE_SOURCE_DIR}/.git")
        set(DEFAULT_BUILD_TYPE "RelWithDebInfo")
    else ()
        set(DEFAULT_BUILD_TYPE "Release")
    endif ()
    set(CMAKE_BUILD_TYPE
            "${DEFAULT_BUILD_TYPE}"
            CACHE
            STRING
            "Choose the type of build, options are: Debug Release RelWithDebInfo MinSizeRel."
            FORCE)
    set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS "Debug" "Release"
            "RelWithDebInfo" "MinSizeRel")
endif ()

include(FindCompiler)
set(CMAKE_CXX_STANDARD 17)

# Set cmake policies
include(DalPolicy)
dal_policy()

# Project name and version should be set after cmake_policy CMP0048
set(PROJECT_VERSION 0.1.0)
project(
        dal
        VERSION ${PROJECT_VERSION}
        LANGUAGES C CXX)

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
find_llvm()

# Export compile-cli_command.json for clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Add subdirectories
add_subdirectory(src)
add_subdirectory(tests)