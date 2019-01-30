cmake_minimum_required(VERSION 3.5)
project(isis-supervisor-api VERSION 0.1.2 LANGUAGES C)

set(kubos_hal_dir "${isis-supervisor-api_SOURCE_DIR}/../../hal/kubos-hal/")
add_subdirectory("${kubos_hal_dir}" "${CMAKE_BINARY_DIR}/kubos-hal-build")

add_library(isis-supervisor-api
  source/checksum.c
  source/supervisor.c
)

target_include_directories(isis-supervisor-api
  PUBLIC
  "${isis-supervisor-api_SOURCE_DIR}/isis-iobc-supervisor"
)

target_link_libraries(isis-supervisor-api
  kubos-hal
)
