# FreeRTOS CMake build
cmake_minimum_required (VERSION 2.8)
project (freertos)

add_library (freertos STATIC
             FreeRTOS/Source/croutine.c
             FreeRTOS/Source/event_groups.c
             FreeRTOS/Source/list.c
             FreeRTOS/Source/queue.c
             FreeRTOS/Source/tasks.c
             FreeRTOS/Source/timers.c)

set (FREERTOS_INCLUDES ${CMAKE_CURRENT_LIST_DIR}/FreeRTOS/Source/include)
set (PORTABLE_DIR FreeRTOS/Source/portable)

# TODO allow for heap impl customization
target_sources(freertos PRIVATE ${PORTABLE_DIR}/MemMang/heap_3.c)

if (TARGET_LIKE_GCC)
    set (GCC_DIR ${PORTABLE_DIR}/GCC)
    if (TARGET_LIKE_CORTEX_M4)
        set (PORT_DIR ${CMAKE_CURRENT_LIST_DIR}/${GCC_DIR}/ARM_CM4F)
        target_sources (freertos PRIVATE ${PORT_DIR}/port.c)
        target_include_directories (freertos PUBLIC ${PORT_DIR})
        list(APPEND FREERTOS_INCLUDES ${PORT_DIR})
        target_link_libraries (freertos cmsis-core)
    elseif (TARGET_LIKE_MSP430_GCC)
        set (GCC_DIR ${PORTABLE_DIR}/GCC)
        set (PORT_DIR ${CMAKE_CURRENT_LIST_DIR}/${GCC_DIR}/MSP430F449)
        list(APPEND FREERTOS_INCLUDES ${PORT_DIR})
        target_sources (freertos PRIVATE ${PORT_DIR}/port.c)
        target_include_directories (freertos PUBLIC ${PORT_DIR} ${FREERTOS_INCLUDES})
        target_link_libraries(freertos putchar)
    endif()
endif()

target_include_directories (freertos PUBLIC ${FREERTOS_INCLUDES})
set_property(GLOBAL APPEND PROPERTY YOTTA_GLOBAL_INCLUDE_DIRS ${FREERTOS_INCLUDES})

