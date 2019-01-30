cmake_minimum_required(VERSION 3.5)
project(isis-imtq-test)

set(imtq_api_dir "${CMAKE_SOURCE_DIR}/../../../../apis/isis-imtq-api/")
add_subdirectory("${imtq_api_dir}" "${CMAKE_BINARY_DIR}/imtq-api-build")

add_executable(isis-imtq-test source/main.c)

target_link_libraries(isis-imtq-test isis-imtq-api)

target_include_directories(isis-imtq-test PUBLIC "${imtq_api_dir}/isis-imtq-api")
