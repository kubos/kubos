cmake_minimum_required(VERSION 3.5)
project(isis-imtq-test VERSION 0.1.0)

set(imtq_api_dir "${isis-imtq-test_SOURCE_DIR}/../../../../apis/isis-imtq-api/")
add_subdirectory("${imtq_api_dir}" "${CMAKE_BINARY_DIR}/imtq-api-build")

add_executable(isis-imtq-test source/main.c)

target_link_libraries(isis-imtq-test isis-imtq-api)

target_include_directories(isis-imtq-test
  PRIVATE "${imtq_api_dir}/isis-imtq-api"
)
