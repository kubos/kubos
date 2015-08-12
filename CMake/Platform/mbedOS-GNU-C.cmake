# Copyright (C) 2014-2015 ARM Limited. All rights reserved.
#message("mbedOS-GNU-C.cmake included")

# Override the link rules:
set(CMAKE_C_CREATE_SHARED_LIBRARY "echo 'shared libraries not supported' && 1")
set(CMAKE_C_CREATE_SHARED_MODULE  "echo 'shared modules not supported' && 1")
set(CMAKE_C_CREATE_STATIC_LIBRARY "<CMAKE_AR> -cr <LINK_FLAGS> <TARGET> <OBJECTS>")
set(CMAKE_C_COMPILE_OBJECT        "<CMAKE_C_COMPILER> <DEFINES> <FLAGS> -o <OBJECT> -c <SOURCE>")
# <LINK_LIBRARIES> is grouped with system libraries so that system library
# functions (e.g. malloc) can be overridden by symbols in <LINK_LIBRARIES>
set(CMAKE_C_LINK_EXECUTABLE       "<CMAKE_C_COMPILER> <CMAKE_C_LINK_FLAGS> <LINK_FLAGS> -Wl,-Map,<TARGET>.map -Wl,--start-group <OBJECTS> <LINK_LIBRARIES> -lm -lc -lgcc -lm -lc -lgcc -Wl,--end-group  --specs=nano.specs -o <TARGET>")


set(CMAKE_C_FLAGS_DEBUG_INIT          "-g")
set(CMAKE_C_FLAGS_MINSIZEREL_INIT     "-Os -DNDEBUG")
set(CMAKE_C_FLAGS_RELEASE_INIT        "-Os -DNDEBUG")
set(CMAKE_C_FLAGS_RELWITHDEBINFO_INIT "-Os -g -DNDEBUG")
set(CMAKE_INCLUDE_SYSTEM_FLAG_C "-isystem ")


set(CMAKE_ASM_FLAGS_DEBUG_INIT          "-g")
set(CMAKE_ASM_FLAGS_MINSIZEREL_INIT     "-Os -DNDEBUG")
set(CMAKE_ASM_FLAGS_RELEASE_INIT        "-Os -DNDEBUG")
set(CMAKE_ASM_FLAGS_RELWITHDEBINFO_INIT "-Os -g -DNDEBUG")
set(CMAKE_INCLUDE_SYSTEM_FLAG_ASM  "-isystem ")
