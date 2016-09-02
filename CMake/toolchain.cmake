# Original mbed-gcc toolchain code copyright (C) 2014-2015 ARM Limited.
# Modifications copyright (C) 2016 Kubos Corporation

if(TARGET_KUBOS_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_KUBOS_GCC_TOOLCHAIN_INCLUDED 1)

# search path for included .cmake files (set this as early as possible, so that
# indirect includes still use it)
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}")

set(CMAKE_SYSTEM_NAME KubOS)
set(CMAKE_SYSTEM_VERSION 1)

# required for -include yotta_config.h
set(YOTTA_FORCE_INCLUDE_FLAG "-include")

# provide compatibility definitions for compiling with this target: these are
# definitions that legacy code assumes will be defined.
add_definitions("-DTOOLCHAIN_GCC")

macro(_gcc_not_found progname)
    message("**************************************************************************\n")
    message(" ERROR: the gcc program \"${progname}\" could not be found\n")
    if(COMMAND gcc_not_found)
        gcc_not_found()
    endif()
    message("\n**************************************************************************")
    message(FATAL_ERROR "missing program prevents build")
    return()
endmacro()

# provide a macro to downstream targets to use a custom gcc prefix
macro(gcc_load_toolchain prefix)
    # find the compiler and associated tools that we need:
    find_program(K_GCC "${prefix}gcc")
    find_program(K_GPP "${prefix}g++")
    find_program(K_OBJCOPY "${prefix}objcopy")
    if(NOT K_GCC)
        _gcc_not_found("${prefix}gcc")
    endif()
    if(NOT K_GPP)
        _gcc_not_found("${prefix}g++")
    endif()
    if(NOT K_OBJCOPY)
        _gcc_not_found("${prefix}objcopy")
    endif()

    # set default compilation flags
    IF(CMAKE_BUILD_TYPE MATCHES Debug)
        set(_C_FAMILY_FLAGS_INIT "-fno-exceptions -fno-unwind-tables -ffunction-sections -fdata-sections -Wall -Wextra -gstrict-dwarf")
    ELSE()
        set(_C_FAMILY_FLAGS_INIT "-fno-exceptions -fno-unwind-tables -ffunction-sections -fdata-sections -Wextra -gstrict-dwarf")
    ENDIF()

    set(CMAKE_C_FLAGS_INIT   "${_C_FAMILY_FLAGS_INIT}")
    set(CMAKE_ASM_FLAGS_INIT "-fno-exceptions -fno-unwind-tables -x assembler-with-cpp")
    set(CMAKE_CXX_FLAGS_INIT "--std=gnu++11 ${_C_FAMILY_FLAGS_INIT} -fno-rtti -fno-threadsafe-statics")
    set(CMAKE_MODULE_LINKER_FLAGS_INIT
        "-fno-exceptions -fno-unwind-tables -Wl,--gc-sections -Wl,--sort-common -Wl,--sort-section=alignment"
    )
    if((NOT DEFINED YOTTA_CFG_GCC_PRINTF_FLOAT) OR (YOTTA_CFG_GCC_PRINTF_FLOAT))
        set(CMAKE_EXE_LINKER_FLAGS_INIT "${CMAKE_EXE_LINKER_FLAGS_INIT} -Wl,-u,_printf_float")
    endif()

    # Set the compiler to ARM-GCC
    if(CMAKE_VERSION VERSION_LESS "3.6.0")
        include(CMakeForceCompiler)
        cmake_force_c_compiler("${K_GCC}" GNU)
        cmake_force_cxx_compiler("${K_GPP}" GNU)
    else()
        # from 3.5 the force_compiler macro is deprecated: CMake can detect
        # arm-none-eabi-gcc as being a GNU compiler automatically
        set(CMAKE_C_COMPILER "${K_GCC}")
        set(CMAKE_CXX_COMPILER "${K_GPP}")
    endif()
endmacro()
