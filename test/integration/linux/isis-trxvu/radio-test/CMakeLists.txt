cmake_minimum_required(VERSION 3.5)
project(isis-trxvu-radio-test)

set(trxvu_api_dir "${isis-trxvu-radio-test_SOURCE_DIR}/../../../../../apis/isis-trxvu-api/")
add_subdirectory("${trxvu_api_dir}" "${CMAKE_BINARY_DIR}/trxvu-api-build")

add_executable(isis-trxvu-radio-test
  source/main.c)

target_link_libraries(isis-trxvu-radio-test isis-trxvu-api)
