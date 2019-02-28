cmake_minimum_required(VERSION 3.5)

add_executable(json-test-run-construction run-construction.c)

target_link_libraries(json-test-run-construction json)

enable_testing()
add_test(json-test-run-construction json-test-run-construction)
