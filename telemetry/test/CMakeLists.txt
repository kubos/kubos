message("custom test build")

# add include path definitions needed only for tests (from testDependencies)
include_directories("../yotta_modules/unity")

# define the tests themselves:

add_executable(telemetry-test-telemetry "telemetry.c" "stubs/ipc.c")
target_link_libraries(telemetry-test-telemetry
    unity
    telemetry
)

set_target_properties(telemetry-test-telemetry
        PROPERTIES
        LINK_FLAGS  
        "-Wl,--wrap=subscriber_read -Wl,--wrap=subscriber_connect -Wl,--wrap=send_csp"
)

yotta_postprocess_target(EXECUTABLE telemetry-test-telemetry)
add_test(telemetry-test-telemetry telemetry-test-telemetry)
add_dependencies(all_tests telemetry-test-telemetry)