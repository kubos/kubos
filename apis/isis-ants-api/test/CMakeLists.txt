cmake_minimum_required(VERSION 3.5)
project(isis-ants-test)

set(cmocka_dir "${isis-ants-test_SOURCE_DIR}/../../../cmocka/")
add_subdirectory("${cmocka_dir}" "${CMAKE_BINARY_DIR}/cmocka-build")

set(ants_api_dir "${isis-ants-test_SOURCE_DIR}/..")
add_subdirectory("${ants_api_dir}" "${CMAKE_BINARY_DIR}/ants-api-build")

add_executable(isis-ants-api-test
  antenna/antenna.c
  antenna/sysfs.c)

set_target_properties(isis-ants-api-test
        PROPERTIES
        LINK_FLAGS
        "-Wl,--wrap=open \
         -Wl,--wrap=close \
         -Wl,--wrap=ioctl \
         -Wl,--wrap=write \
         -Wl,--wrap=read")

target_link_libraries(isis-ants-api-test
  cmocka
  isis-ants-api
  kubos-hal
  pthread
)

target_include_directories(isis-ants-api-test
  PRIVATE "${ants_api_dir}/isis-ants-api"
  PRIVATE "${cmocka_dir}/cmocka-1.1.0/include"
)

enable_testing()
add_test(isis-ants-api-test isis-ants-api-test)
