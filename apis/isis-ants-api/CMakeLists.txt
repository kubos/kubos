cmake_minimum_required(VERSION 3.5)
project(isis-ants-api VERSION 0.1.2 LANGUAGES C)

set(kubos_hal_dir "${isis-ants-api_SOURCE_DIR}/../../hal/kubos-hal/")
add_subdirectory("${kubos_hal_dir}" "${CMAKE_BINARY_DIR}/kubos-hal-build")

add_library(isis-ants-api
  source/ants.c
)

target_include_directories(isis-ants-api
  SYSTEM
  PUBLIC
  "${isis-ants-api_SOURCE_DIR}/isis-ants-api"
)

target_link_libraries(isis-ants-api
  kubos-hal
  pthread
)

# add_subdirectory(test)
