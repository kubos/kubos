set_target_properties(telemetry-storage-test-telemetry-storage
        PROPERTIES
        LINK_FLAGS  
        "-Wl,--wrap=disk_save_string"
)
