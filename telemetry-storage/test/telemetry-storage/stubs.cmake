set_target_properties(telemetry-storage-test-telemetry-storage
        PROPERTIES
        LINK_FLAGS  
        "-Wl,--wrap=klog_init_file \ 
	 -Wl,--wrap=KLOG_TELEMETRY \
	 -Wl,--wrap=klog_cleanup" 
)
