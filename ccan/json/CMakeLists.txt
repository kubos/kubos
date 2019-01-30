cmake_minimum_required(VERSION 3.5)
project(json VERSION 0.1.2 LANGUAGES C)

add_library(json
  source/json.c
)

target_include_directories(json
  SYSTEM
  PUBLIC
  "${json_SOURCE_DIR}/json"
)
