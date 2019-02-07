cmake_minimum_required(VERSION 3.5)
project(isis-ants-api VERSION 1.0.0)

set(kubos_hal_dir "${isis-ants-api_SOURCE_DIR}/../../hal/kubos-hal/")
add_subdirectory("${kubos_hal_dir}" "${CMAKE_BINARY_DIR}/kubos-hal-build")

add_library(isis-ants-api
  source/ants.c
)

target_include_directories(isis-ants-api
  PUBLIC "${isis-ants-api_SOURCE_DIR}/isis-ants-api"
)

target_link_libraries(isis-ants-api
  kubos-hal
  pthread
)
