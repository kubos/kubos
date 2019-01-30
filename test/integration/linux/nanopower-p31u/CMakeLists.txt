cmake_minimum_required(VERSION 3.5)
project(nanopower-p31u)

set(nanopower_api_dir "${nanopower-p31u_SOURCE_DIR}/../../../../apis/gomspace-p31u-api/")
add_subdirectory("${nanopower_api_dir}" "${CMAKE_BINARY_DIR}/nanopower-api-build")

add_executable(nanopower-p31u
  source/main.c)

target_include_directories(nanopower-p31u
  PRIVATE
  "${nanopower_api_dir}/gomspace-p31u-api"
)

target_link_libraries(nanopower-p31u gomspace-p31u-api)
