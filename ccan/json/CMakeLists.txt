cmake_minimum_required(VERSION 3.5)
project(json VERSION 1.0.0)

add_library(json
  source/json.c
)

target_include_directories(json
  SYSTEM
  PUBLIC
  "${json_SOURCE_DIR}/json"
)
