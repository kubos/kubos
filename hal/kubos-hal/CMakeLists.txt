cmake_minimum_required(VERSION 3.5)
project(kubos-hal VERSION 0.1.2 LANGUAGES C)

add_library(kubos-hal
  source/i2c.c
)

target_include_directories(kubos-hal
  SYSTEM
  PUBLIC
  "${kubos-hal_SOURCE_DIR}/kubos-hal"
)
