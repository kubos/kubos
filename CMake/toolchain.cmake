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

# post-process elf files into .bin files:
set(YOTTA_POSTPROCESS_COMMAND "arm-none-eabi-objcopy -O binary YOTTA_CURRENT_EXE_NAME YOTTA_CURRENT_EXE_NAME.bin")


# set default compilation flags
set(_C_FAMILY_FLAGS_INIT "-fno-exceptions -fno-unwind-tables -ffunction-sections -fdata-sections -Wall -Wextra")
set(CMAKE_C_FLAGS_INIT   "-std=c99 ${_C_FAMILY_FLAGS_INIT}")
set(CMAKE_ASM_FLAGS_INIT "-fno-exceptions -fno-unwind-tables -x assembler-with-cpp")
set(CMAKE_CXX_FLAGS_INIT "${_C_FAMILY_FLAGS_INIT}")
set(CMAKE_MODULE_LINKER_FLAGS_INIT
    "-fno-exceptions -fno-unwind-tables -Wl,--gc-sections -Wl,--sort-common -Wl,--sort-section=alignment"
)
set(CMAKE_EXE_LINKER_FLAGS_INIT "${CMAKE_MODULE_LINKER_FLAGS_INIT} -Wl,-wrap,main") 

# Set the compiler to ARM-GCC
include(CMakeForceCompiler)

cmake_force_c_compiler(arm-none-eabi-gcc GNU)
cmake_force_cxx_compiler(arm-none-eabi-g++ GNU)

message("CMAKE_EXE_LINKER_FLAGS_INIT (base): ${CMAKE_EXE_LINKER_FLAGS_INIT}")
message("CMAKE_EXE_LINKER_FLAGS (base): ${CMAKE_EXE_LINKER_FLAGS}")
