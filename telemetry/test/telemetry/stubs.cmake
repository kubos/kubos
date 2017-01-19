set_target_properties(telemetry-test-telemetry
        PROPERTIES
        LINK_FLAGS  
        "-Wl,--wrap=kprv_subscriber_read -Wl,--wrap=kprv_subscriber_connect -Wl,--wrap=kprv_send_csp"
)