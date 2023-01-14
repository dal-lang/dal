# Find LLVM
macro(find_llvm)
    find_package(LLVM REQUIRED CONFIG)
    # LLVM 15.0.5 minimum required
    if (LLVM_PACKAGE_VERSION VERSION_LESS 15.0.5)
        message(FATAL_ERROR "LLVM 15.0.5 or newer is required")
    endif ()
    message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
    message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")
    include_directories(${LLVM_INCLUDE_DIRS})
    add_definitions(${LLVM_DEFINITIONS})
    llvm_map_components_to_libnames(
            llvm_libs
            ${LLVM_TARGETS_TO_BUILD}
            support
            core
            irreader
            codegen
            mc
            mcparser
            option
    )
endmacro()