# Copyright (C) 2014-2015 ARM Limited. All rights reserved. 

if(TARGET_MBED_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_MBED_GCC_TOOLCHAIN_INCLUDED 1)

# search path for included .cmake files (set this as early as possible, so that
# indirect includes still use it)
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}")

include(CMakeForceCompiler)

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

# post-process elf files into .bin files:
set(YOTTA_POSTPROCESS_COMMAND "arm-none-eabi-objcopy -O binary YOTTA_CURRENT_EXE_NAME YOTTA_CURRENT_EXE_NAME.bin")


# set default compilation flags
set(_C_FAMILY_FLAGS_INIT "-fno-threadsafe-statics -fno-exceptions -fno-unwind-tables -ffunction-sections -fdata-sections -Wall -Wextra")
set(CMAKE_C_FLAGS_INIT   "-std=c99 ${_C_FAMILY_FLAGS_INIT}")
set(CMAKE_ASM_FLAGS_INIT "-fno-exceptions -fno-unwind-tables -x assembler-with-cpp")
set(CMAKE_CXX_FLAGS_INIT "${_C_FAMILY_FLAGS_INIT} -fno-rtti")
set(CMAKE_MODULE_LINKER_FLAGS_INIT
    "-fno-exceptions -fno-unwind-tables -Wl,--gc-sections -Wl,--sort-common -Wl,--sort-section=alignment"
)
set(CMAKE_EXE_LINKER_FLAGS_INIT "${CMAKE_MODULE_LINKER_FLAGS_INIT} -Wl,-wrap,main") 

# Set the compiler to ARM-GCC
include(CMakeForceCompiler)

cmake_force_c_compiler(arm-none-eabi-gcc GNU)
cmake_force_cxx_compiler(arm-none-eabi-g++ GNU)

# post-process elf files into .bin files:
function(yotta_apply_target_rules target_type target_name)
    if(${target_type} STREQUAL "EXECUTABLE")
        add_custom_command(TARGET ${target_name}
            POST_BUILD
            COMMAND arm-none-eabi-objcopy -O binary ${target_name} ${target_name}.bin
            COMMENT "converting to .bin"
            VERBATIM
        )
    endif()
endfunction()

