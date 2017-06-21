/*
* Copyright (C) 2017 Kubos Corporation
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

#pragma once

#include <csp/csp.h>
#include <command-and-control/types.h>
#include <tinycbor/cbor.h>

typedef int (*lib_function)(int, char**);

bool cnc_daemon_encode_processing_error(uint8_t * data, CNCWrapper * result, CborEncoder * encoder, CborEncoder * container);

bool cnc_daemon_encode_response(uint8_t * data, CNCWrapper * wrapper, CborEncoder * encoder, CborEncoder * container);

bool cnc_daemon_finish_encode_response_and_send(uint8_t * data, CborEncoder *encoder, CborEncoder * container);

bool cnc_daemon_get_command(csp_socket_t* sock, char * command);

bool cnc_daemon_parse (char * args, CNCWrapper * my_arguments);

bool cnc_daemon_parse_buffer(CNCWrapper * wrapper, CborDataWrapper * data_wrapper);

bool cnc_daemon_parse_buffer_from_packet(csp_packet_t * packet, CborDataWrapper * data_wrapper);

bool cnc_daemon_parse_command(CborParser * parser, CborValue * map, CNCWrapper * wrapper);

bool cnc_daemon_load_and_run_command(CNCWrapper * wrapper);

bool cnc_daemon_send_buffer(uint8_t * data, size_t data_len);

bool cnc_daemon_start_encode_response(int message_type, CNCWrapper * wrapper);

bool cnc_daemon_send_result(CNCWrapper * wrapper);


#define LIB_FORMAT_STR "/%s"

#ifndef YOTTA_CFG_CNC_REGISTRY_DIR
#define CNC_REGISTRY_DIR "/root"
#else
#define CNC_REGISTRY_DIR YOTTA_CFG_CNC_REGISTRY_DIR
#endif

#define MODULE_REGISTRY_DIR CNC_REGISTRY_DIR LIB_FORMAT_STR

#ifdef YOTTA_CFG_CNC_DAEMON_CMD_STR_LEN
#define CMD_STR_LEN YOTTA_CFG_CNC_DAEMON_CMD_STR_LEN
#else
#define CMD_STR_LEN 75
#endif

#ifdef YOTTA_CFG_CNC_DAEMON_SO_PATH_LENGTH
#define SO_PATH_LENGTH YOTTA_CFG_CNC_DAEMON_SO_PATH_LENGTH
#else
#define SO_PATH_LENGTH 75
#endif

#ifdef YOTTA_CFG_CNC_DAEMON_LOG_PATH
#define DAEMON_LOG_PATH YOTTA_CFG_CNC_DAEMON_LOG_PATH
#else
#define DAEMON_LOG_PATH "/var/log/daemon.log"
#endif

