IF(TARGET_MSP430_GCC_TOOLCHAIN_INCLUDED)
    return()
endif()
set(TARGET_MSP430_GCC_TOOLCHAIN_INCLUDED 1)

# find the compiler and associated tools that we need:
find_program(MSP430_GCC msp430-gcc)
find_program(MSP430_GPP msp430-g++)
find_program(MSP430_OBJCOPY msp430-objcopy)
macro(gcc_program_notfound progname)
    message("**************************************************************************\n")
    message(" ERROR: the arm gcc program ${progname} could not be found\n")
    endif()
    message("\n**************************************************************************")
    message(FATAL_ERROR "missing program prevents build")
    return()
endmacro(gcc_program_notfound)

if(NOT MSP430_GCC)
    gcc_program_notfound("msp430-gcc")
endif()
if(NOT MSP430_GPP)
    gcc_program_notfound("msp430-g++")
endif()
if(NOT MSP430_OBJCOPY)
    gcc_program_notfound("msp430-objcopy")
endif()

# Set the compiler to msp430-gcc
if(CMAKE_VERSION VERSION_LESS "3.6.0")
    include(CMakeForceCompiler)
    cmake_force_c_compiler("${MSP430_GCC}" GNU)
    cmake_force_cxx_compiler("${MSP430_GPP}" GNU)
else()
    # from 3.5 the force_compiler macro is deprecated: CMake can detect
    # arm-none-eabi-gcc as being a GNU compiler automatically
    set(CMAKE_C_COMPILER "${MSP430_GCC}")
    set(CMAKE_CXX_COMPILER "${MSP430_GPP}")
endif()

set(_CPU_COMPILATION_OPTIONS "-mmcu=msp430f5529")
set(_CPU_DEFINES "")


set(CMAKE_C_FLAGS_INIT             "${CMAKE_C_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS} ${_CPU_DEFINES}" CACHE STRING "" FORCE)
set(CMAKE_ASM_FLAGS_INIT           "${CMAKE_ASM_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS} ${_CPU_DEFINES}" CACHE STRING "" FORCE)
set(CMAKE_CXX_FLAGS_INIT           "${CMAKE_CXX_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS} ${_CPU_DEFINES}" CACHE STRING "" FORCE)
set(CMAKE_MODULE_LINKER_FLAGS_INIT "${CMAKE_MODULE_LINKER_FLAGS_INIT} ${_CPU_COMPILATION_OPTIONS}" CACHE STRING "" FORCE)
set(CMAKE_EXE_LINKER_FLAGS_INIT    "${CMAKE_EXE_LINKER_FLAGS_INIT} -T\"${CMAKE_CURRENT_LIST_DIR}/../ld/memory.x\" -T\"${CMAKE_CURRENT_LIST_DIR}/../ld/periph.x\" -T\"${CMAKE_CURRENT_LIST_DIR}/../ld/msp430.x\"" CACHE STRING "" FORCE)
