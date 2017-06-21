if(TARGET_MSP430_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_MSP430_GCC_TOOLCHAIN_INCLUDED 1)

set(_CPU_COMPILATION_OPTIONS "-mmcu=msp430f5529 -std=gnu99")
set(_CPU_DEFINES "")

set(MEMORY_X "${CMAKE_CURRENT_LIST_DIR}/../ld/memory.x")

# Users can force demarcation of the 2K usb ram segment in the linker script
# at the 0x1c00 address that is normally claimed by the entire 10K "ram" segment
# Example in a Kubos application's config.json (at the project root):
#
# { "kubos-build": { "msp430-usbram": true } }

if(YOTTA_CFG_KUBOS_BUILD_MSP430_USBRAM)
    set(MEMORY_X "${CMAKE_CURRENT_LIST_DIR}/../ld/memory_usbram.x")
endif()

set(CMAKE_C_FLAGS_INIT             "${CMAKE_C_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS} ${_CPU_DEFINES}")
set(CMAKE_ASM_FLAGS_INIT           "${CMAKE_ASM_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS} ${_CPU_DEFINES}")
set(CMAKE_CXX_FLAGS_INIT           "${CMAKE_CXX_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS} ${_CPU_DEFINES}")
set(CMAKE_MODULE_LINKER_FLAGS_INIT "${CMAKE_MODULE_LINKER_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS}")
set(CMAKE_C_LINK_FLAGS             "${CMAKE_C_LINK_FLAGS} ${_CPU_COMPILATION_OPTIONS}")
set(CMAKE_EXE_LINKER_FLAGS_INIT    "${CMAKE_EXE_LINKER_FLAGS_INIT} -T\"${MEMORY_X}\" -T\"${CMAKE_CURRENT_LIST_DIR}/../ld/periph.x\" -T\"${CMAKE_CURRENT_LIST_DIR}/../ld/msp430.x\"")