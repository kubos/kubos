# Copyright (C) 2017 Kubos Corporation
if (TARGET_BBB_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_BBB_GCC_TOOLCHAIN_INCLUDED 1)

set(CMAKE_SYSTEM_PROCESSOR "am335x")

macro(gcc_not_found)
    message("/usr/bin/bbb_toolchain has not been found.  It can be installed by building Linux")
    message("with the BR2_HOST_DIR option set to '/usr/bin/bbb_toolchain'")
endmacro()

gcc_load_toolchain("bbb_toolchain/usr/bin/arm-linux-")