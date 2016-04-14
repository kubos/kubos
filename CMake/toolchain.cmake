# Copyright (C) 2016 Kubos Corporation
if (TARGET_KUBOS_ARM_NONE_EABI_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_KUBOS_ARM_NONE_EABI_GCC_TOOLCHAIN_INCLUDED 1)

set(CMAKE_SYSTEM_PROCESSOR "armv7-m")
add_definitions("-DTOOLCHAIN_GCC_ARM")

macro(gcc_not_found)
    if(CMAKE_HOST_SYSTEM_NAME STREQUAL "Windows" OR CMAKE_HOST_SYSTEM_NAME STREQUAL "Linux")
        message(" you can install the ARM GCC embedded compiler tools from:")
        message(" https://launchpad.net/gcc-arm-embedded/+download ")
    elseif(CMAKE_HOST_SYSTEM_NAME STREQUAL "Darwin")
        message(" it is included in the arm-none-eabi-gcc package that you can install")
        message(" with homebrew:\n")
        message("   brew tap ARMmbed/homebrew-formulae")
        message("   brew install arm-none-eabi-gcc")
    endif()
endmacro()

gcc_load_toolchain("arm-none-eabi-")
