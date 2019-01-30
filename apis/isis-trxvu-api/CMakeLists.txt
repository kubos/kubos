cmake_minimum_required(VERSION 3.5)
project(isis-trxvu-api VERSION 0.1.2 LANGUAGES C)

set(kubos_hal_dir "${isis-trxvu-api_SOURCE_DIR}/../../hal/kubos-hal/")
add_subdirectory("${kubos_hal_dir}" "${CMAKE_BINARY_DIR}/kubos-hal-build")

add_library(isis-trxvu-api
  source/radio_core.c
  source/radio_rx.c
  source/radio_tx.c
)

target_include_directories(isis-trxvu-api
  SYSTEM
  PUBLIC
  "${isis-trxvu-api_SOURCE_DIR}/isis-trxvu-api"
)

target_link_libraries(isis-trxvu-api
  kubos-hal
  pthread
  m
)
