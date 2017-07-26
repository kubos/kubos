# Copyright (C) 2017 Kubos Corporation
if (TARGET_KUBOS_LINUX_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_KUBOS_LINUX_GCC_TOOLCHAIN_INCLUDED 1)

# Common settings for KubOS Linux targets
set(_C_FAMILY_FLAGS_INIT   "-std=c99 ${_C_FAMILY_FLAGS_INIT}")