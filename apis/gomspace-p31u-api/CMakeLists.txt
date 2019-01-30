cmake_minimum_required(VERSION 3.5)
project(gomspace-p31u-api VERSION 0.1.2 LANGUAGES C)

set(kubos_hal_dir "${gomspace-p31u-api_SOURCE_DIR}/../../hal/kubos-hal/")
add_subdirectory("${kubos_hal_dir}" "${CMAKE_BINARY_DIR}/kubos-hal-build")

add_library(gomspace-p31u-api
  source/nanopower.c
)

target_include_directories(gomspace-p31u-api
  SYSTEM
  PUBLIC
  "${gomspace-p31u-api_SOURCE_DIR}/gomspace-p31u-api"
)

target_link_libraries(gomspace-p31u-api
  kubos-hal
  pthread
)
