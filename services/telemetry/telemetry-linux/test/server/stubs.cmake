set_target_properties(telemetry-linux-test-server
        PROPERTIES
        LINK_FLAGS  
        "-Wl,--wrap=kprv_send_csp \
         -Wl,--wrap=kprv_subscriber_socket_connect \
         -Wl,--wrap=csp_close \
         -Wl,--wrap=kprv_subscriber_read"
)