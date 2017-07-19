# Copyright (C) 2017 Kubos Corporation
if (TARGET_KUBOS_LINUX_BEAGLEBONE_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_KUBOS_LINUX_BEAGLEBONE_GCC_TOOLCHAIN_INCLUDED 1)

add_definitions("-DTOOLCHAIN_GCC_ARM")

gcc_load_toolchain("arm-linux-gnueabihf-")

set(CMAKE_C_LINK_FLAGS   "-static")