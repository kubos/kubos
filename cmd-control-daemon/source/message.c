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
#include "cmd-control-daemon/logging.h"

bool cnc_daemon_send_result(CNCWrapper * wrapper)
{
    if (wrapper == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Received a NULL pointer internally. Aborting encoding..\n");
        KLOG_DEBUG(&log_handle, LOG_COMPONENT_NAME, "cnc_daemon_send_result called with a null wrapper pointer\n");
        return false;
    }

    if (wrapper->err) //Thinking of changing the err flag to a state enum or similar multi-state member type
    {
        KLOG_INFO(&log_handle, LOG_COMPONENT_NAME, "Encoding result of type: Processing Error\n");
        cnc_daemon_start_encode_response(RESPONSE_TYPE_PROCESSING_ERROR, wrapper);
    }
    else
    {
        KLOG_INFO(&log_handle, LOG_COMPONENT_NAME, "Encoding result of type: Command Result\n");
        cnc_daemon_start_encode_response(RESPONSE_TYPE_COMMAND_RESULT, wrapper);
    }
}


bool cnc_daemon_start_encode_response(int message_type, CNCWrapper * wrapper)
{
    CborEncoder encoder, container;
    CborError err;
    uint8_t data[MTU] = {0};

    if (wrapper == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Received a NULL pointer internally. Aborting encoding..\n");
        KLOG_DEBUG(&log_handle, LOG_COMPONENT_NAME, "cnc_daemon_start_encode_response called with a null wrapper pointer\n");
        return false;
    }

    cbor_encoder_init(&encoder, data, MTU, 0);
    err = cbor_encoder_create_map(&encoder, &container, 4); //TODO: Dynamically assign map size
    if (err)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to initialize cbor encoder, Error code: %i", err);
        return false;
    }

    if (err = cbor_encode_text_stringz(&container, "MSG_TYPE"))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode key MSG_TYPE. Error code: %i\n", err);
        return false;
    }

    if (err = cbor_encode_int(&container, message_type))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode value for key \"MSG_TYPE\" Error code: %i\n", err);
        return false;
    }

    switch (message_type)
    {
        case RESPONSE_TYPE_COMMAND_RESULT:
            return cnc_daemon_encode_response(data, wrapper, &encoder, &container);
        case RESPONSE_TYPE_PROCESSING_ERROR:
            return cnc_daemon_encode_processing_error(data, wrapper, &encoder, &container);
    }
}


bool cnc_daemon_encode_response(uint8_t * data, CNCWrapper * wrapper, CborEncoder * encoder, CborEncoder * container)
{
    CborError err;

    if(data == NULL || wrapper == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Received a NULL pointer internally. Aborting encoding..\n");
        KLOG_DEBUG(&log_handle, LOG_COMPONENT_NAME, "cnc_daemon_encode_response called with a null pointer\n");
        return false;
    }

    if (err = cbor_encode_text_stringz(container, "RETURN_CODE"))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode key \"RETURN_CODE\". Error code: %i\n", err);
        return false;
    }

    if (err = cbor_encode_simple_value(container, wrapper->response_packet->return_code))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode value for key \"RETURN_CODE\". Error code:%i\n", err);
        return false;
    }

    if (err = cbor_encode_text_stringz(container, "EXEC_TIME"))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode key \"EXEC_TIME\". Error code: %i\n", err);
        return false;
    }

    if (err = cbor_encode_double(container, wrapper->response_packet->execution_time))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode value for key \"EXEC_TIME\". Error code:%i\n", err);
        return false;
    }

    if (err = cbor_encode_text_stringz(container, "OUTPUT"))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode key \"OUTPUT\". Error code: %i\n", err);
        return false;
    }

    if (err = cbor_encode_text_stringz(container, wrapper->response_packet->output))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode value for key \"OUTPUT\". Error code:%i\n", err);
        return false;
    }

    return cnc_daemon_finish_encode_response_and_send(data, encoder, container);
}


bool cnc_daemon_encode_processing_error(uint8_t * data, CNCWrapper * result, CborEncoder * encoder, CborEncoder * container)
{
    CborError err;

    if (data == NULL || result == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Called with NULL pointer. Aborting encoding\n");
        KLOG_DEBUG(&log_handle, LOG_COMPONENT_NAME, "cnc_daemon_encode_processing_error called with a null pointer\n");
        return false;
    }

    if (err = cbor_encode_text_stringz(container, "ERROR_MSG"))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode key \"ERROR_MSG\" Error code: %i\n", err);
        return false;
    }

    if (err = cbor_encode_text_stringz(container, result->output))
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Unable to encode value for key \"ERROR_MSG\" Error code: %i\n", err);
        return false;
    }

    return cnc_daemon_finish_encode_response_and_send(data, encoder, container);
}


bool cnc_daemon_finish_encode_response_and_send(uint8_t * data, CborEncoder *encoder, CborEncoder * container)
{
    if (data == NULL)
    {
        KLOG_ERR(&log_handle, LOG_COMPONENT_NAME, "Called with NULL pointer. Aborting encoding..\n");
        KLOG_DEBUG(&log_handle, LOG_COMPONENT_NAME, "cnc_daemon_encode_processing_error called with a data pointer\n");
        return false;
    }

    cbor_encoder_close_container(encoder, container);
    size_t data_len = cbor_encoder_get_buffer_size(encoder, data);

    KLOG_INFO(&log_handle, LOG_COMPONENT_NAME, "Encoded response buffer size: %lu\n", data_len);

    return cnc_daemon_send_buffer(data, data_len);
}

