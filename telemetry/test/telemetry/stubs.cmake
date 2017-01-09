set_target_properties(telemetry-test-telemetry
        PROPERTIES
        LINK_FLAGS  
        "-Wl,--wrap=subscriber_read -Wl,--wrap=subscriber_connect -Wl,--wrap=send_csp"
)