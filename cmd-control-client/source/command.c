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

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "command-and-control/types.h"
#include "cmd-control-client/client.h"
#include "tinycbor/cbor.h"


bool encode_packet(CborDataWrapper * data_wrapper, CNCCommandPacket * packet)
{
    start_encode_response(MESSAGE_TYPE_COMMAND_INPUT, data_wrapper, packet);
}


bool start_encode_response(int message_type, CborDataWrapper * data_wrapper, CNCCommandPacket * packet)
{
    CborEncoder encoder, container;
    CborError err;

    if (data_wrapper == NULL || packet == NULL)
    {
        return false;
    }

    cbor_encoder_init(&encoder, data_wrapper->data, MTU, 0);
    err = cbor_encoder_create_map(&encoder, &container, 6); //TODO: Dynamically assign map size
    if (err)
    {
        return false;
    }

    err = cbor_encode_text_stringz(&container, "MSG_TYPE");
    if (err || cbor_encode_int(&container, message_type))
    {
        return false;
    }

    switch (message_type)
    {
        case MESSAGE_TYPE_COMMAND_INPUT:
            return encode_command(data_wrapper, packet, &encoder, &container);
            break;
    }
}


bool encode_command(CborDataWrapper * data_wrapper, CNCCommandPacket * packet, CborEncoder * encoder, CborEncoder * container)
{
    CborError err;

    if(data_wrapper == NULL || packet == NULL)
    {
        return false;
    }

    err = cbor_encode_text_stringz(container, "ACTION");
    if (err || cbor_encode_int(container, (int)packet->action))
    {
        return false;
    }

    err = cbor_encode_text_stringz(container, "ARG_COUNT");
    if (err || cbor_encode_int(container, packet->arg_count))
    {
        return false;
    }

    err = cbor_encode_text_stringz(container, "COMMAND_NAME");
    if (err || cbor_encode_text_stringz(container, packet->cmd_name))
    {
        return false;
    }

    //TODO:Encode multiple args
    err = cbor_encode_text_stringz(container, "ARGS");
    if (err || cbor_encode_text_stringz(container, packet->args[0]))
    {
        return false;
    }

    return finish_encode_response_and_send(data_wrapper, encoder, container);
}


bool finish_encode_response_and_send(CborDataWrapper * data_wrapper, CborEncoder *encoder, CborEncoder * container)
{
    if (data_wrapper == NULL)
    {
        return false;
    }

    cbor_encoder_close_container(encoder, container);
    data_wrapper->length = cbor_encoder_get_buffer_size(encoder, data_wrapper->data);
    /*printf("Data: %s\n", data_wrapper->data);*/
    return true;
}

