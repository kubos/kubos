# cmocka CMake build
cmake_minimum_required (VERSION 2.8)
project (cmocka)
include (CheckIncludeFiles)

add_definitions(-DHAVE_CONFIG_H=1)

set(YOTTA_MODULE_NAME cmocka)

add_library (cmocka STATIC
             cmocka-1.1.0/src/cmocka.c)

target_include_directories (cmocka PUBLIC cmocka-1.1.0/include PUBLIC ${CMAKE_CURRENT_BINARY_DIR})

option(HAVE_SIGNAL_H "" ON)
option(HAVE_STDARG_H "" ON)
option(HAVE_SETJMP_H "" ON)
option(HAVE_MALLOC_H "" ON)
option(HAVE_INTTYPES_H "" ON)
configure_file(config.h.cmake ${CMAKE_CURRENT_LIST_DIR}/cmocka-1.1.0/src/config.h)