set_target_properties(telemetry - linux - test - client
                                                     PROPERTIES
                                                         LINK_FLAGS
                      "-Wl,--wrap=kprv_socket_send \
         -Wl,--wrap=kprv_socket_client_connect \
         -Wl,--wrap=kprv_socket_recv")