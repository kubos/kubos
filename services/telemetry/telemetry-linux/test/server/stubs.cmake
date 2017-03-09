set_target_properties(telemetry-linux-test-server
        PROPERTIES
        LINK_FLAGS  
        "-Wl,--wrap=kprv_socket_send \
         -Wl,--wrap=kprv_socket_client_connect \
         -Wl,--wrap=kprv_socket_recv \
         -Wl,--wrap=kprv_socket_close"
)