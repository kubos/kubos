cmake_minimum_required(VERSION 3.5)
project(iobc-supervisor-test)

set(supervisor_api_dir "${iobc-supervisor-test_SOURCE_DIR}/../../../../apis/isis-iobc-supervisor/")
add_subdirectory("${supervisor_api_dir}" "${CMAKE_BINARY_DIR}/supervisor-api-build")

add_executable(iobc-supervisor-test
  source/main.c)

target_link_libraries(iobc-supervisor-test isis-supervisor-api)

target_include_directories(iobc-supervisor-test PUBLIC  "${supervisor_api_dir}/isis-iobc-supervisor")
