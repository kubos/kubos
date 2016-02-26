# Copyright (C) 2014-2015 ARM Limited. All rights reserved. 

if(TARGET_MBED_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_MBED_GCC_TOOLCHAIN_INCLUDED 1)

# search path for included .cmake files (set this as early as possible, so that
# indirect includes still use it)
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}")

set(CMAKE_SYSTEM_NAME mbedOS)
set(CMAKE_SYSTEM_VERSION 1)
set(CMAKE_SYSTEM_PROCESSOR "armv7-m")

# required for -include yotta_config.h
set(YOTTA_FORCE_INCLUDE_FLAG "-include")

# legacy definitions for building mbed 2.0 modules with a retrofitted build
# system:
set(MBED_LEGACY_TOOLCHAIN "GCC_ARM")
# provide compatibility definitions for compiling with this target: these are
# definitions that legacy code assumes will be defined. 
add_definitions("-DTOOLCHAIN_GCC -DTOOLCHAIN_GCC_ARM -DMBED_OPERATORS")


# find the compiler and associated tools that we need:
find_program(ARM_NONE_EABI_GCC arm-none-eabi-gcc)
find_program(ARM_NONE_EABI_GPP arm-none-eabi-g++)
find_program(ARM_NONE_EABI_OBJCOPY arm-none-eabi-objcopy)
macro(gcc_program_notfound progname)
    message("**************************************************************************\n")
    message(" ERROR: the arm gcc program ${progname} could not be found\n")
    if(CMAKE_HOST_SYSTEM_NAME STREQUAL "Windows" OR CMAKE_HOST_SYSTEM_NAME STREQUAL "Linux")
        message(" you can install the ARM GCC embedded compiler tools from:")
        message(" https://launchpad.net/gcc-arm-embedded/+download ")
    elseif(CMAKE_HOST_SYSTEM_NAME STREQUAL "Darwin")
        message(" it is included in the arm-none-eabi-gcc package that you can install")
        message(" with homebrew:\n")
        message("   brew tap ARMmbed/homebrew-formulae")
        message("   brew install arm-none-eabi-gcc")
    endif()
    message("\n**************************************************************************")
    message(FATAL_ERROR "missing program prevents build")
    return()
endmacro(gcc_program_notfound)

if(NOT ARM_NONE_EABI_GCC)
    gcc_program_notfound("arm-none-eabi-gcc")
endif()
if(NOT ARM_NONE_EABI_GPP)
    gcc_program_notfound("arm-none-eabi-g++")
endif()
if(NOT ARM_NONE_EABI_OBJCOPY)
    gcc_program_notfound("arm-none-eabi-objcopy")
endif()


# post-process elf files into .bin files (legacy backwards-compatible command):
set(YOTTA_POSTPROCESS_COMMAND "\"${ARM_NONE_EABI_OBJCOPY}\" -O binary YOTTA_CURRENT_EXE_NAME YOTTA_CURRENT_EXE_NAME.bin")


# set default compilation flags
set(_C_FAMILY_FLAGS_INIT "-fno-exceptions -fno-unwind-tables -ffunction-sections -fdata-sections -Wall -Wextra")
set(CMAKE_C_FLAGS_INIT   "-std=c99 ${_C_FAMILY_FLAGS_INIT}")
set(CMAKE_ASM_FLAGS_INIT "-fno-exceptions -fno-unwind-tables -x assembler-with-cpp")
set(CMAKE_CXX_FLAGS_INIT "--std=gnu++11 ${_C_FAMILY_FLAGS_INIT} -fno-rtti -fno-threadsafe-statics")
set(CMAKE_MODULE_LINKER_FLAGS_INIT
    "-fno-exceptions -fno-unwind-tables -Wl,--gc-sections -Wl,--sort-common -Wl,--sort-section=alignment"
)
set(CMAKE_EXE_LINKER_FLAGS_INIT "${CMAKE_MODULE_LINKER_FLAGS_INIT} -Wl,-wrap,main") 
if((NOT DEFINED YOTTA_CFG_GCC_PRINTF_FLOAT) OR (YOTTA_CFG_GCC_PRINTF_FLOAT))
    set(CMAKE_EXE_LINKER_FLAGS_INIT "${CMAKE_EXE_LINKER_FLAGS_INIT} -Wl,-u,_printf_float")
endif()

# Set the compiler to ARM-GCC
if(CMAKE_VERSION VERSION_LESS "3.5.0")
    include(CMakeForceCompiler)
    cmake_force_c_compiler("${ARM_NONE_EABI_GCC}" GNU)
    cmake_force_cxx_compiler("${ARM_NONE_EABI_GPP}" GNU)
else()
    # from 3.5 the force_compiler macro is deprecated: CMake can detect
    # arm-none-eabi-gcc as being a GNU compiler automatically
    set(CMAKE_C_COMPILER "${ARM_NONE_EABI_GCC}")
    set(CMAKE_CXX_COMPILER "${ARM_NONE_EABI_GPP}")
endif()

# post-process elf files into .bin files:
function(yotta_apply_target_rules target_type target_name)
    if(${target_type} STREQUAL "EXECUTABLE")
        add_custom_command(TARGET ${target_name}
            POST_BUILD
            COMMAND "${ARM_NONE_EABI_OBJCOPY}" -O binary ${target_name} ${target_name}.bin
            COMMENT "converting to .bin"
            VERBATIM
        )
    endif()
endfunction()

