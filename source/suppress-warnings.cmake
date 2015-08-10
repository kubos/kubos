message("suppressing warnings from mbed-hal-st-stm32cubef4")

set_target_properties(mbed-hal-st-stm32cubef4
    PROPERTIES COMPILE_FLAGS "-Wno-implicit-function-declaration -Wno-unused-parameter"
)
