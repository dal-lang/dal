find_program(C clang++)
if (NOT C)
    find_program(C clang++-15)
endif ()
if (NOT C)
    message(FATAL_ERROR "Could not find clang++")
endif ()

set(CMAKE_CXX_COMPILER ${C})


