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
set(PROJECT_VERSION "0.1.0")
project(
        dal
        VERSION ${PROJECT_VERSION}
        LANGUAGES C CXX)

# Configure config.h
configure_file("${CMAKE_SOURCE_DIR}/cmake/templates/config.h.in"
        "${CMAKE_BINARY_DIR}/config.h")

# Find llvm
include(FindLLVM)
find_llvm()

# GTest
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

# Export compile-command.json for clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Add subdirectories
add_subdirectory(src)
