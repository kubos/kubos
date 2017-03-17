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

#include <command-and-control/types.h>
#include <tinycbor/cbor.h>
#include "cmd-control-daemon/daemon.h"

bool cnc_daemon_parse_buffer_from_packet(csp_packet_t * packet, CborDataWrapper * data_wrapper)
{
    if (packet == NULL || data_wrapper == NULL)
    {
        return false;
    }
    data_wrapper->length = packet->length;
    memcpy(data_wrapper->data, packet->data, data_wrapper->length);
    return true;
}

bool cnc_daemon_parse_buffer(CNCWrapper * wrapper, CborDataWrapper * data_wrapper)
{
    CborParser parser;
    CborValue map, element;
    int message_type;

    CborError err = cbor_parser_init(data_wrapper->data, data_wrapper->length, 0, &parser, &map);
    if (err)
    {
        return false;
    }

    err = cbor_value_map_find_value(&map, "MSG_TYPE", &element);
    if (err || cbor_value_get_int(&element, &message_type))
    {
        return false;
    }

    switch (message_type)
    {
        case MESSAGE_TYPE_COMMAND_INPUT:
            return cnc_daemon_parse_command(&parser, &map, wrapper);
            break;
        default:
            fprintf(stderr, "Received unknown message type: %i\n", message_type);
            return false;
   }
}


bool cnc_daemon_parse_command(CborParser * parser, CborValue * map, CNCWrapper * wrapper)
{
    size_t len;
    uint8_t return_code;
    double execution_time;
    char output[MTU];

    CborValue element;
    CborError err;
    int i;
    err = cbor_value_map_find_value(map, "ACTION", &element);
    if (err || cbor_value_get_int(&element, &i))
    {
        return false;
    }
    wrapper->command_packet->action = (CNCAction) i;

    err = cbor_value_map_find_value(map, "ARG_COUNT", &element);
    if (err || cbor_value_get_int(&element, &(wrapper->command_packet->arg_count)))
    {
        return false;
    }

    len = CMD_PACKET_CMD_NAME_LEN;
    err = cbor_value_map_find_value(map, "COMMAND_NAME", &element);
    if (err || cbor_value_copy_text_string(&element, &(wrapper->command_packet->cmd_name), &len, NULL))
    {
        return false;
    }

    len = CMD_PACKET_ARG_LEN;
    err = cbor_value_map_find_value(map, "ARGS", &element);
    if (err || cbor_value_copy_text_string(&element, &(wrapper->command_packet->args[0]), &len, NULL))
    {
        return false;
    }
    return true;

}

