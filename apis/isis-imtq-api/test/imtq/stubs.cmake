set_target_properties(isis-imtq-api-test-imtq
        PROPERTIES
        LINK_FLAGS
        "-Wl,--wrap=open \
         -Wl,--wrap=close \
         -Wl,--wrap=ioctl \
         -Wl,--wrap=write \
         -Wl,--wrap=read")