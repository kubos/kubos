cmake_minimum_required(VERSION 3.5)
project(adc-thermistor)

add_executable(adc-thermistor
  source/main.c)

target_link_libraries(adc-thermistor m)
