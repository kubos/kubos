cmake_minimum_required(VERSION 3.5)
project(kubos-hal-tests)

set(cmocka_dir "${kubos-hal-tests_SOURCE_DIR}/../../../cmocka/")
add_subdirectory("${cmocka_dir}" "${CMAKE_BINARY_DIR}/cmocka-build")

set(hal_dir "${kubos-hal-tests_SOURCE_DIR}/../")
add_subdirectory("${hal_dir}" "${CMAKE_BINARY_DIR}/hal-build")

add_executable(kubos-hal-test-i2c
  i2c/i2c.c
  i2c/sysfs.c)

target_include_directories(kubos-hal-test-i2c
  PRIVATE
  "${cmocka_dir}/cmocka-1.1.0/include"
  PRIVATE
  "${hal_dir}/kubos-hal"
)

set_target_properties(kubos-hal-test-i2c
        PROPERTIES
        LINK_FLAGS
        "-Wl,--wrap=open \
         -Wl,--wrap=close \
         -Wl,--wrap=ioctl \
         -Wl,--wrap=write \
         -Wl,--wrap=read")

target_link_libraries(kubos-hal-test-i2c
  cmocka
  kubos-hal
)

add_test(kubos-hal-test-i2c kubos-hal-test-i2c)

enable_testing()
