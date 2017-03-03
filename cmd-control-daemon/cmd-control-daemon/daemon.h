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

#ifndef DAEMON_H
#define DAEMON_H
#include <csp/csp.h>
#include <command-and-control/types.h>
#include <tinycbor/cbor.h>

#ifdef YOTTA_CFG_CNC_DAEMON_CMD_STR_LEN
#define CMD_STR_LEN YOTTA_CFG_CNC_DAEMON_CMD_STR_LEN
#else
#define CMD_STR_LEN 75
#endif

typedef int (*lib_function)(int, char**);

bool encode_processing_error(uint8_t * data, CNCWrapper * result, CborEncoder * encoder, CborEncoder * container);

bool encode_response(uint8_t * data, CNCWrapper * wrapper, CborEncoder * encoder, CborEncoder * container);

bool finish_encode_response_and_send(uint8_t * data, CborEncoder *encoder, CborEncoder * container);

bool get_command(csp_socket_t* sock, char * command);

bool parse (char * args, CNCWrapper * my_arguments);

bool parse_command_cbor(csp_packet_t * packet, char * command);

bool process_and_run_command(CNCWrapper * wrapper);

bool send_buffer(uint8_t * data, size_t data_len);

bool start_encode_response(int message_type, CNCWrapper * wrapper);

bool send_result(CNCWrapper * wrapper);

#endif
