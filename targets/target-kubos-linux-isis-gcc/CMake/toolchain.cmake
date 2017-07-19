# Copyright (C) 2016 Kubos Corporation
if (TARGET_KUBOS_LINUX_ISIS_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_KUBOS_LINUX_ISIS_GCC_TOOLCHAIN_INCLUDED 1)

set(CMAKE_SYSTEM_PROCESSOR "arm926ej-s")

macro(gcc_not_found)
    message("/usr/bin/iobc_toolchain has not been found.  It can be installed by building Linux")
	message("with the BR2_HOST_DIR option set to '/usr/bin/iobc_toolchain'")
endmacro()

gcc_load_toolchain("iobc_toolchain/usr/bin/arm-linux-")