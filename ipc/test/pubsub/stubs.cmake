set_target_properties(ipc-test-pubsub
        PROPERTIES
        LINK_FLAGS  
        "-Wl,--wrap=csp_read \
         -Wl,--wrap=csp_accept \
         -Wl,--wrap=csp_socket \
         -Wl,--wrap=csp_bind \
         -Wl,--wrap=csp_listen \
         -Wl,--wrap=csp_connect \
         -Wl,--wrap=csp_send \
         -Wl,--wrap=csp_buffer_get \
         -Wl,--wrap=csp_buffer_free \
         -Wl,--wrap=csp_conn_dport \
         -Wl,--wrap=csp_conn_sport \
         -Wl,--wrap=csp_service_handler"
)