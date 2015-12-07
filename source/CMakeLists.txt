# mbed Microcontroller Library
# Copyright (c) 2014 ARM Limited
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

# this custom build file pulls in different startup assembly based on the
# toolchain being used


if(${CMAKE_C_COMPILER_ID} STREQUAL GNU)
    set(STARTUP_ASM "bootstrap_gcc/startup_stm32f429xx.s")
    set(STARTUP_SRC "")
else()
    set(STARTUP_ASM "")
    set(STARTUP_SRC "")
    message(FATAL_ERROR "Unsupported target toolchain (gcc and armcc are supported)")
endif()



enable_language(ASM)
set_property(SOURCE ${STARTUP_ASM} PROPERTY LANGUAGE ASM)

add_library(mbed-hal-st-stm32f429zi
    ${STARTUP_ASM}
    ${STARTUP_SRC}
    "PeripheralPins.c"
)
set_target_properties(mbed-hal-st-stm32f429zi PROPERTIES
    LINKER_LANGUAGE C
)

# uvisor-lib is only a dependency if we're building for gcc
if(TARGET_LIKE_GCC)
    set(EXTRA_LINK_LIBS uvisor-lib)
else()
    set(EXTRA_LINK_LIBS)
endif()

#TODO: target dependencies should not be set here
target_link_libraries(mbed-hal-st-stm32f429zi
    ${EXTRA_LINK_LIBS}
    mbed-hal
)
