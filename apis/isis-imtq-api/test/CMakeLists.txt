cmake_minimum_required(VERSION 3.5)
project(imtq-test)

set(cmocka_dir "${imtq-test_SOURCE_DIR}/../../../cmocka/")
add_subdirectory("${cmocka_dir}" "${CMAKE_BINARY_DIR}/cmocka-build")

set(imtq_api_dir "${imtq-test_SOURCE_DIR}/..")
add_subdirectory("${imtq_api_dir}" "${CMAKE_BINARY_DIR}/imtq-api-build")

add_executable(isis-imtq-api-imtq-test
  imtq/imtq.c
  imtq/sysfs.c)

set_target_properties(isis-imtq-api-imtq-test
        PROPERTIES
        LINK_FLAGS
        "-Wl,--wrap=open \
         -Wl,--wrap=close \
         -Wl,--wrap=ioctl \
         -Wl,--wrap=write \
         -Wl,--wrap=read")

target_link_libraries(isis-imtq-api-imtq-test
  cmocka
  isis-imtq-api
  kubos-hal
  pthread
)

target_include_directories(isis-imtq-api-imtq-test
  PRIVATE "${cmocka_dir}/cmocka-1.1.0/include"
)

add_test(isis-imtq-api-imtq-test isis-imtq-api-imtq-test)

add_executable(isis-imtq-api-adcs-test
  adcs/adcs.c
  adcs/sysfs.c)

set_target_properties(isis-imtq-api-adcs-test
        PROPERTIES
        LINK_FLAGS
        "-Wl,--wrap=open \
         -Wl,--wrap=close \
         -Wl,--wrap=ioctl \
         -Wl,--wrap=write \
         -Wl,--wrap=read")

target_link_libraries(isis-imtq-api-adcs-test
  cmocka
  isis-imtq-api
  kubos-hal
  pthread
)

target_include_directories(isis-imtq-api-adcs-test
  PRIVATE "${cmocka_dir}/cmocka-1.1.0/include"
)

add_test(isis-imtq-api-adcs-test isis-imtq-api-adcs-test)

enable_testing()
