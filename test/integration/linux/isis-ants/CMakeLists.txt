cmake_minimum_required(VERSION 3.5)
project(isis-ants)

set(ants_api_dir "${isis-ants_SOURCE_DIR}/../../../../apis/isis-ants-api/")
add_subdirectory("${ants_api_dir}" "${CMAKE_BINARY_DIR}/ants-api-build")

add_executable(isis-ants
  source/main.c)

target_link_libraries(isis-ants isis-ants-api)
