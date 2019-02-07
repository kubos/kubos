cmake_minimum_required(VERSION 3.5)
project(gomspace-p31u-api-test)

set(cmocka_dir "${gomspace-p31u-api-test_SOURCE_DIR}/../../../cmocka/")
add_subdirectory("${cmocka_dir}" "${CMAKE_BINARY_DIR}/cmocka-build")

set(nanopower_api_dir "${gomspace-p31u-api-test_SOURCE_DIR}/..")
add_subdirectory("${nanopower_api_dir}" "${CMAKE_BINARY_DIR}/nanopower-api-build")

add_executable(gomspace-p31u-api-nanopower-test
  nanopower/nanopower.c
  nanopower/sysfs.c)

set_target_properties(gomspace-p31u-api-nanopower-test
        PROPERTIES
        LINK_FLAGS
        "-Wl,--wrap=open \
         -Wl,--wrap=close \
         -Wl,--wrap=ioctl \
         -Wl,--wrap=write \
         -Wl,--wrap=read")

target_include_directories(gomspace-p31u-api-nanopower-test
  PRIVATE "${nanopower_api_dir}/gomspace-p31u-api"
)

target_link_libraries(gomspace-p31u-api-nanopower-test
  cmocka
  gomspace-p31u-api
  kubos-hal
  pthread
)

add_test(gomspace-p31u-api-nanopower-test gomspace-p31u-api-nanopower-test)
enable_testing()
