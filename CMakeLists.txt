cmake_minimum_required (VERSION 2.6)
project (kubos-core)
include (CheckIncludeFiles)
set (CSP_VERSION "1.4.0" CACHE STRING "")

add_library (kubos-core STATIC
             source/unity/unity.c
             source/arch/k_alloc_csp.c
             source/arch/k_alloc_malloc.c
             source/modules/klog/klog.c
             source/modules/ham/aprs.c
             source/modules/ham/ax25.c
             source/modules/gps/gps.c
             source/modules/gps/nmea.c)


if (TARGET_LIKE_LINUX)
    set (KUBOS_CORE_POSIX ON)
    set (KUBOS_CORE_ARCH "posix")
elseif (TARGET_LIKE_FREERTOS)
    set (KUBOS_CORE_FREERTOS ON)
    set (KUBOS_CORE_ARCH "freertos")
else ()
    message (FATAL_ERROR "Please set or implement correct architecture!!!")
endif ()

file (GLOB_RECURSE ARCH_FILES source/arch/${KUBOS_CORE_ARCH}/*.c)
target_sources (kubos-core PUBLIC ${ARCH_FILES})

target_link_libraries(kubos-core
    csp
)

## Demos

add_executable (uart_demo demos/uart_demo.c)
target_link_libraries(uart_demo kubos-core)

add_executable (location_demo demos/location_demo.c)
target_link_libraries(location_demo kubos-core)

add_executable (csp_simple demos/csp_simple.c)
target_link_libraries(csp_simple kubos-core)

## Tests

add_executable (kubos-core-test-aprs test/aprs.c)
target_link_libraries(kubos-core-test-aprs kubos-core)
add_test(kubos-core-test-aprs kubos-core-test-aprs)

add_executable (kubos-core-test-ax25 test/ax25.c)
target_link_libraries(kubos-core-test-ax25 kubos-core)
add_test(kubos-core-test-ax25 kubos-core-test-ax25)

add_executable (kubos-core-test-klog test/klog.c)
target_link_libraries(kubos-core-test-klog kubos-core)
add_test(kubos-core-test-klog kubos-core-test-klog)

add_executable (kubos-core-test-nmea test/nmea.c)
target_link_libraries(kubos-core-test-nmea kubos-core)
add_test(kubos-core-test-nmea kubos-core-test-nmea)
