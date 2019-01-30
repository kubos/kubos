cmake_minimum_required(VERSION 3.5)
project(lsm303dlhc-i2c)

set(kubos_hal_dir "${lsm303dlhc-i2c_SOURCE_DIR}/../../../../hal/kubos-hal/")
add_subdirectory("${kubos_hal_dir}" "${CMAKE_BINARY_DIR}/kubos-hal-build")

add_executable(lsm303dlhc-i2c
  source/main.c)

target_include_directories(lsm303dlhc-i2c
  PRIVATE
  "${kubos_hal_dir}/kubos-hal"
  )

target_link_libraries(lsm303dlhc-i2c kubos-hal)
