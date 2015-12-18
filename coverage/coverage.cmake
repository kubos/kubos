
string(TOUPPER "${YOTTA_MODULE_NAME}" upper_name)
string(REPLACE "-" "_" under_name "${upper_name}")

if(${YOTTA_CFG_DEBUG_OPTIONS_COVERAGE_MODULES_${under_name}})
    message("Code coverage enabled on ${YOTTA_MODULE_NAME}")
    get_property(s TARGET ${YOTTA_MODULE_NAME} PROPERTY COMPILE_FLAGS SET)
    if(${s})
        get_target_property(flags ${YOTTA_MODULE_NAME} COMPILE_FLAGS)
    endif()
    set_target_properties(${YOTTA_MODULE_NAME} PROPERTIES COMPILE_FLAGS "${flags} -fprofile-arcs -ftest-coverage")
endif()
