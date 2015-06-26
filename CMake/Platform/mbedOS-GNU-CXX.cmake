# Copyright (C) 2014-2015 ARM Limited. All rights reserved.
#message("mbedOS-GNU-CXX.cmake included")

# can't test the compiler because it cross-compiles
set(CMAKE_CXX_COMPILER_WORKS TRUE)


execute_process(
    COMMAND "${CMAKE_CXX_COMPILER}" "--version"
    OUTPUT_VARIABLE _ARM_GNU_GCC_VERSION_OUTPUT
)
string(REGEX REPLACE ".* ([0-9]+[.][0-9]+[.][0-9]+) .*" "\\1" _ARM_GNU_GCC_VERSION "${_ARM_GNU_GCC_VERSION_OUTPUT}")
message("GCC version is: ${_ARM_GNU_GCC_VERSION}")

set(CMAKE_CXX_CREATE_SHARED_LIBRARY "echo 'shared libraries not supported' && 1")
set(CMAKE_CXX_CREATE_SHARED_MODULE  "echo 'shared modules not supported' && 1")
set(CMAKE_CXX_CREATE_STATIC_LIBRARY "<CMAKE_AR> -cr <LINK_FLAGS> <TARGET> <OBJECTS>")
set(CMAKE_CXX_COMPILE_OBJECT        "<CMAKE_CXX_COMPILER> <DEFINES> <FLAGS> -o <OBJECT> -c <SOURCE>")
set(CMAKE_CXX_LINK_EXECUTABLE       "<CMAKE_CXX_COMPILER> <CMAKE_CXX_LINK_FLAGS> <LINK_FLAGS> <OBJECTS> <LINK_LIBRARIES> -Wl,-Map,<TARGET>.map -Wl,--start-group -lnosys -lstdc++ -lsupc++ -lm -lc -lgcc -lnosys -lnosys -lstdc++ -lsupc++ -lm -lc -lgcc -lnosys -Wl,--end-group  --specs=nano.specs -o <TARGET>")

set(CMAKE_CXX_FLAGS_DEBUG_INIT          "-g")
set(CMAKE_CXX_FLAGS_MINSIZEREL_INIT     "-Os -DNDEBUG")
set(CMAKE_CXX_FLAGS_RELEASE_INIT        "-Os -DNDEBUG")
set(CMAKE_CXX_FLAGS_RELWITHDEBINFO_INIT "-Os -g -DNDEBUG")
set(CMAKE_INCLUDE_SYSTEM_FLAG_CXX "-isystem ")

