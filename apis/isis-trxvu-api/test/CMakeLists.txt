cmake_minimum_required(VERSION 3.5)
project(trxvu-test)

set(cmocka_dir "${trxvu-test_SOURCE_DIR}/../../../cmocka/")
add_subdirectory("${cmocka_dir}" "${CMAKE_BINARY_DIR}/cmocka-build")

set(trxvu_api_dir "${trxvu-test_SOURCE_DIR}/..")
add_subdirectory("${trxvu_api_dir}" "${CMAKE_BINARY_DIR}/trxvu-api-build")

add_executable(isis-trxvu-api-radio-test
  radio/radio.c
  radio/sysfs.c)

set_target_properties(isis-trxvu-api-radio-test
        PROPERTIES
        LINK_FLAGS
        "-Wl,--wrap=open \
         -Wl,--wrap=close \
         -Wl,--wrap=ioctl \
         -Wl,--wrap=write \
         -Wl,--wrap=read")

target_link_libraries(isis-trxvu-api-radio-test
  cmocka
  isis-trxvu-api
)

target_include_directories(isis-trxvu-api-radio-test
  PRIVATE "${trxvu_api_dir}/isis-trxvu-api"
  PRIVATE "${cmocka_dir}/cmocka-1.1.0/include"
)

enable_testing()
add_test(isis-trxvu-api-radio-test isis-trxvu-api-radio-test)
