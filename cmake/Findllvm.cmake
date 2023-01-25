set(LLVM_CONFIG_ERROR_MSG "")
while (1)
    unset(LLVM_CONFIG_EXECUTABLE CACHE)
    find_program(LLVM_CONFIG_EXECUTABLE
            NAMES llvm-config-15 llvm-config-15.0 llvm-config150 llvm-config15 llvm-config NAMES_PER_DIR)
    if (NOT LLVM_CONFIG_EXECUTABLE)
        if (NOT LLVM_CONFIG_ERROR_MSG STREQUAL "")
            list(JOIN LLVM_CONFIG_ERROR_MSG "\n" LLVM_CONFIG_ERROR_MESSAGE)
            message(FATAL_ERROR ${LLVM_CONFIG_ERROR_MESSAGE})
        else ()
            message(FATAL_ERROR "llvm-config not found")
        endif ()
    endif ()

    # Check if the version of llvm-config is compatible with the version of LLVM
    # that we are building against.
    execute_process(COMMAND ${LLVM_CONFIG_EXECUTABLE} --version
            OUTPUT_VARIABLE LLVM_CONFIG_VERSION
            OUTPUT_STRIP_TRAILING_WHITESPACE)

    get_filename_component(LLVM_CONFIG_DIR "${LLVM_CONFIG_EXECUTABLE}" DIRECTORY)
    if ("${LLVM_CONFIG_VERSION}" VERSION_LESS 15 VERSION_EQUAL 16 OR "${LLVM_CONFIG_VERSION}" VERSION_GREATER 16)
        list(APPEND LLVM_CONFIG_ERROR_MSG "expected LLVM 15.x but found ${LLVM_CONFIG_VERSION} using ${LLVM_CONFIG_EXECUTABLE}")

        # Ignore this directory and try the search again
        list(APPEND CMAKE_IGNORE_PATH "${LLVM_CONFIG_DIR}")
        continue()
    endif ()

    # Check if llvm supports linking as static library.
    set(STATIC_LINK "--link-static")
    execute_process(
            COMMAND ${LLVM_CONFIG_EXECUTABLE} --libs ${STATIC_LINK}
            OUTPUT_QUIET
            ERROR_VARIABLE LLVM_CONFIG_ERROR
            ERROR_STRIP_TRAILING_WHITESPACE
    )
    if (LLVM_CONFIG_ERROR)
        list(APPEND LLVM_CONFIG_ERROR_MSG "LLVM 15.x found at ${LLVM_CONFIG_EXECUTABLE} does not support static linking")
        list(APPEND CMAKE_IGNORE_PATH "${LLVM_CONFIG_DIR}")
        continue()
    endif ()

    execute_process(
            COMMAND ${LLVM_CONFIG_EXECUTABLE} --targets-built
            OUTPUT_VARIABLE LLVM_TARGETS_BUILT_SPACES
            OUTPUT_STRIP_TRAILING_WHITESPACE
    )
    string(REPLACE " " ";" LLVM_TARGETS_BUILT "${LLVM_TARGETS_BUILT_SPACES}")
    set(DAL_LLVM_REQUIRED_TARGETS "AArch64;AMDGPU;ARM;WebAssembly;X86")
    set(DAL_LLVM_REQUIRED_TARGETS_ENABLED TRUE)
    foreach (TARGET IN LISTS ${DAL_LLVM_REQUIRED_TARGETS})
        list(FIND LLVM_TARGETS_BUILT "${TARGET}" TARGET_INDEX)
        if (${TARGET_INDEX} EQUAL -1)
            list(APPEND LLVM_CONFIG_ERROR_MESSAGES "LLVM (according to ${LLVM_CONFIG_EXECUTABLE}) is missing target ${TARGET}.")

            # Ignore this directory and try the search again
            list(APPEND CMAKE_IGNORE_PATH "${LLVM_CONFIG_DIR}")
            set(DAL_LLVM_REQUIRED_TARGETS_ENABLED FALSE)
            break()
        endif ()
    endforeach ()

    if (NOT DAL_LLVM_REQUIRED_TARGETS_ENABLED)
        continue()
    endif ()

    break()
endwhile ()

execute_process(
        COMMAND ${LLVM_CONFIG_EXECUTABLE} --libfiles ${STATIC_LINK}
        OUTPUT_VARIABLE LLVM_LIBRARIES_SPACES
        OUTPUT_STRIP_TRAILING_WHITESPACE)
string(REPLACE " " ";" LLVM_LIBRARIES "${LLVM_LIBRARIES_SPACES}")

execute_process(
        COMMAND ${LLVM_CONFIG_EXECUTABLE} --libdir ${STATIC_LINK}
        OUTPUT_VARIABLE LLVM_LIBDIRS_SPACES
        OUTPUT_STRIP_TRAILING_WHITESPACE)
string(REPLACE " " ";" LLVM_LIBDIRS "${LLVM_LIBDIRS_SPACES}")

execute_process(
        COMMAND ${LLVM_CONFIG_EXECUTABLE} --system-libs ${STATIC_LINK}
        OUTPUT_VARIABLE LLVM_SYSTEM_LIBS_SPACES
        OUTPUT_STRIP_TRAILING_WHITESPACE)
string(REPLACE " " ";" LLVM_SYSTEM_LIBS "${LLVM_SYSTEM_LIBS_SPACES}")

execute_process(
        COMMAND ${LLVM_CONFIG_EXECUTABLE} --shared-mode ${STATIC_LINK}
        OUTPUT_VARIABLE LLVM_LINK_MODE
        OUTPUT_STRIP_TRAILING_WHITESPACE)

execute_process(
        COMMAND ${LLVM_CONFIG_EXECUTABLE} --includedir
        OUTPUT_VARIABLE LLVM_INCLUDE_DIRS_SPACES
        OUTPUT_STRIP_TRAILING_WHITESPACE)
string(REPLACE " " ";" LLVM_INCLUDE_DIRS "${LLVM_INCLUDE_DIRS_SPACES}")

link_directories("${CMAKE_PREFIX_PATH}/lib")
link_directories("${LLVM_LIBDIRS}")