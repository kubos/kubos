set_target_properties(telemetry-test-telemetry
        PROPERTIES
        LINK_FLAGS  
        "-Wl,--wrap=kprv_subscriber_read \
         -Wl,--wrap=kprv_subscriber_connect \
         -Wl,--wrap=kprv_send_csp \
         -Wl,--wrap=kprv_server_accept \
         -Wl,--wrap=kprv_publisher_read"
)