cmake_minimum_required(VERSION 2.8)

# post-process elf files into .bin files:
function(yotta_apply_target_rules target_type target_name)
    if(${target_type} STREQUAL "EXECUTABLE")
        add_custom_command(TARGET ${target_name}
            POST_BUILD
            COMMAND "${K_OBJCOPY}" -O binary ${target_name} ${target_name}.bin
            COMMENT "converting to .bin"
            VERBATIM
        )
    endif()
endfunction()
