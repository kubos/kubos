cmake_minimum_required(VERSION 3.5)
project(adc-thermistor VERSION 0.1.0)

add_executable(adc-thermistor
  source/main.c)

target_link_libraries(adc-thermistor m)
