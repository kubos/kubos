cmake_minimum_required(VERSION 3.5)
project(isis-imtq-api VERSION 0.1.2 LANGUAGES C)

set(kubos_hal_dir "${isis-imtq-api_SOURCE_DIR}/../../hal/kubos-hal/")
add_subdirectory("${kubos_hal_dir}" "${CMAKE_BINARY_DIR}/kubos-hal-build")

set(json_dir "${isis-imtq-api_SOURCE_DIR}/../../ccan/json/")
add_subdirectory("${json_dir}" "${CMAKE_BINARY_DIR}/json-build")

add_library(isis-imtq-api
  source/imtq-config.c
  source/imtq-core.c
  source/imtq-data.c
  source/imtq-ops.c
  )

target_include_directories(isis-imtq-api
  SYSTEM PUBLIC
  "${isis-imtq-api_SOURCE_DIR}/isis-imtq-api"
)

target_include_directories(isis-imtq-api
  SYSTEM PUBLIC
  "${kubos_hal_dir}/kubos-hal"
  )

target_include_directories(isis-imtq-api
  SYSTEM PUBLIC
  "${json_dir}/json"
)

target_link_libraries(isis-imtq-api
  PRIVATE kubos-hal
  PRIVATE json
  PRIVATE pthread
)

# add_subdirectory(test)
