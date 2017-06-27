cmake_minimum_required (VERSION 2.6)

include (CheckIncludeFiles)
set (CSP_VERSION "1.4.0" CACHE STRING "")

option (CSP_USE_RDP "" OFF)
option (QOS "" OFF)
option (PROMISC "" OFF)
option (CSP_USE_CRC32 "" OFF)
option (HMAC "" OFF)
option (XTEA "" OFF)
option (BINDINGS "" OFF)
option (EXAMPLES "" OFF)
option (DEDUP "" OFF)
option (CSP_VERBOSE "" OFF)
option (IF_I2C "" OFF)
option (IF_KISS "" OFF)
option (IF_CAN "" OFF)
option (IF_SOCKET "" OFF)
option (IF_ZMQHUB "" OFF)
option (CSP_DEBUG "" OFF)
option (CAN_SOCKETCAN "" OFF)

set (FREERTOS "" CACHE STRING "FreeRTOS root dir")
option (INIT_SHUTDOWN "" OFF)

#TODO FREERTOS
set (RDP_MAX_WINDOW "20" CACHE STRING "Set maximum window size for RDP")
set (MAX_BIND_PORT "31" CACHE STRING "Set maximum bindable port")
set (MAX_CONNECTIONS "10" CACHE STRING "Set maximum number of concurrent connections")
set (CONN_QUEUE_LENGTH "100" CACHE STRING "Set maximum number of packets in queue for a connection")
set (ROUTER_QUEUE_LENGTH "10" CACHE STRING "Set maximum number of packets to be queued at the input of the router")
set (PADDING "8" CACHE STRING "Set padding bytes before packet length field")
set (LOGLEVEL "debug" CACHE STRING "Set minimum compile time log level. Must be one of 'error', 'warn', 'info' or 'debug'")
set (RTABLE "static" CACHE STRING "Set routing table type")
set (CONNECTION_SO "0x0000" CACHE STRING "Set outgoing connection socket options, see csp.h for valid values")

execute_process (COMMAND git describe --always
                 OUTPUT_VARIABLE GIT_REV
                 OUTPUT_STRIP_TRAILING_WHITESPACE
                 ERROR_QUIET)

if (YOTTA_CFG_CSP_DEBUG)
    set (CSP_DEBUG ON)
endif()

# check if we want KISS/USART
if (YOTTA_CFG_CSP_USART)
    # set flag
    set (CSP_USE_CRC32 ON)
    set (IF_KISS ON)
endif()

if (YOTTA_CFG_CSP_SOCKET)
    set (IF_SOCKET ON)
endif()

if (YOTTA_CFG_CSP_RDP)
    set (CSP_USE_RDP ON)
endif()

add_library (csp STATIC
    csp_bridge.c
    csp_buffer.c
    csp_conn.c
    $<$<BOOL:${CSP_USE_CRC32}>:csp_crc32.c>
    $<$<BOOL:${CSP_DEBUG}>:csp_debug.c>
    $<$<BOOL:${DEDUP}>:csp_dedup.c>
    csp_endian.c
    csp_iflist.c
    csp_io.c
    csp_port.c
    csp_promisc.c
    csp_qfifo.c
    csp_route.c
    csp_service_handler.c
    csp_services.c
    csp_sfp.c
    $<$<BOOL:${HMAC}>:crypto/csp_hmac.c>
    $<$<OR:$<BOOL:${HMAC}>,$<BOOL:${XTEA}>>:crypto/csp_sha1.c>
    $<$<BOOL:${XTEA}>:crypto/csp_xtea.c>
    $<$<BOOL:${CAN_SOCKETCAN}>:drivers/can/can_socketcan.c>
    $<$<AND:$<BOOL:${TARGET_LIKE_KUBOS_RT}>,$<BOOL:${YOTTA_CFG_CSP_USART}>>:drivers/usart/usart_kubos.c>
    interfaces/csp_if_lo.c
    $<$<BOOL:${IF_KISS}>:interfaces/csp_if_kiss.c>
    $<$<BOOL:${IF_SOCKET}>:interfaces/csp_if_socket.c>
    $<$<BOOL:${IF_SOCKET}>:drivers/socket/socket_linux.c>
    $<$<BOOL:${IF_SOCKET}>:drivers/socket/packet.c>
    rtable/csp_rtable_${RTABLE}.c
    $<$<BOOL:${CSP_USE_RDP}>:transport/csp_rdp.c>
    transport/csp_udp.c
)




target_compile_definitions (csp PRIVATE GIT_REV=\"${GIT_REV}\")
target_include_directories (csp PUBLIC ../include ${CMAKE_CURRENT_BINARY_DIR})

# TODO detect endianness
set (CSP_LITTLE_ENDIAN ON)
if (TARGET_LIKE_FREERTOS)
    set (CSP_FREERTOS ON)
    set (CSP_ARCH "freertos")
    target_include_directories (csp PUBLIC ${FREERTOS_INCLUDES})
elseif (TARGET_LIKE_LINUX)
    set (CSP_POSIX ON)
    set (CSP_ARCH "posix")
    target_link_libraries (csp PRIVATE rt pthread util dl)
elseif (TARGET_LIKE_OSX)
    set (CSP_MACOSX ON)
    set (CSP_ARCH "macosx")
    target_link_libraries (csp PRIVATE pthread)
else()
    message ("TARGET_OS current ${YOTTA_CFG_TARGET_OS}")
    message (FATAL ERROR "Set correct target")
endif()

if (TARGET_LIKE_MSP430_GCC)
    set (MAX_CONNECTIONS "2" CACHE STRING "Set maximum number of concurrent connections")
    set (CONN_QUEUE_LENGTH "10" CACHE STRING "Set maximum number of packets in queue for a connection")
    set (ROUTER_QUEUE_LENGTH "5" CACHE STRING "Set maximum number of packets to be queued at the input of the router")
endif()

file (GLOB_RECURSE ARCH_FILES arch/${CSP_ARCH}/*.c)
target_sources (csp PRIVATE ${ARCH_FILES})

target_compile_options(csp PRIVATE -std=gnu99)

if (LOGLEVEL STREQUAL "debug")
    set (CSP_LOG_LEVEL_DEBUG ON)
endif()

if (LOGLEVEL STREQUAL "debug" OR LOGLEVEL STREQUAL "info")
    set (CSP_LOG_LEVEL_INFO ON)
endif()

if (LOGLEVEL STREQUAL "debug" OR LOGLEVEL STREQUAL "info" OR
    LOGLEVEL STREQUAL "warn")
    set (CSP_LOG_LEVEL_WARN ON)
endif()

if (LOGLEVEL STREQUAL "debug" OR LOGLEVEL STREQUAL "info" OR
    LOGLEVEL STREQUAL "warn" OR LOGLEVEL STREQUAL "error")
    set (CSP_LOG_LEVEL_ERROR ON)
endif()

check_include_files (stdbool.h CSP_HAVE_STDBOOL_H)
configure_file (../csp_autoconfig.h.in csp/csp_autoconfig.h @ONLY)
