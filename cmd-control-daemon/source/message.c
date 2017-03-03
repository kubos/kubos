#include <command-and-control/types.h>
#include <tinycbor/cbor.h>

#include "cmd-control-daemon/daemon.h"

bool send_result(CNCWrapper * wrapper)
{
    if (wrapper->err) //Thinking of changing the err flag to a state enum or similar multi-state member type
    {
        start_encode_response(RESPONSE_TYPE_PROCESSING_ERROR, wrapper);
    }
    else
    {
        start_encode_response(RESPONSE_TYPE_COMMAND_RESULT, wrapper);
    }
}


bool start_encode_response(int message_type, CNCWrapper * wrapper)
{
    CborEncoder encoder, container;
    CborError err;
    uint8_t data[MTU] = {0};

    cbor_encoder_init(&encoder, data, MTU, 0);
    err = cbor_encoder_create_map(&encoder, &container, 4); //TODO: Dynamically assign map size
    if (err)
    {
        return false;
    }

    err = cbor_encode_text_stringz(&container, "MSG_TYPE");
    if (cbor_encode_int(&container, message_type))
    {
        return false;
    }

    switch (message_type)
    {
        case RESPONSE_TYPE_COMMAND_RESULT:
            return encode_response(data, wrapper, &encoder, &container);
        case RESPONSE_TYPE_PROCESSING_ERROR:
            return encode_processing_error(data, wrapper, &encoder, &container);
    }
}


bool encode_response(uint8_t * data, CNCWrapper * wrapper, CborEncoder * encoder, CborEncoder * container)
{
    CborError err;
    err = cbor_encode_text_stringz(container, "RETURN_CODE");
    if (err || cbor_encode_simple_value(container, wrapper->response_packet->return_code))
    {
        return false;
    }

    err = cbor_encode_text_stringz(container, "EXEC_TIME");
    if (err || cbor_encode_double(container, wrapper->response_packet->execution_time))
    {
        return false;
    }

    err = cbor_encode_text_stringz(container, "OUTPUT");
    if (err || cbor_encode_text_stringz(container, wrapper->response_packet->output))
    {
        return false;
    }

    return finish_encode_response_and_send(data, encoder, container);
}


bool encode_processing_error(uint8_t * data, CNCWrapper * result, CborEncoder * encoder, CborEncoder * container)
{
    CborError err;
    err = cbor_encode_text_stringz(container, "ERROR_MSG");
    if (err || cbor_encode_text_stringz(container, result->output))
    {
        return false;
    }

    return finish_encode_response_and_send(data, encoder, container);
}


bool finish_encode_response_and_send(uint8_t * data, CborEncoder *encoder, CborEncoder * container)
{
    cbor_encoder_close_container(encoder, container);
    size_t data_len = cbor_encoder_get_buffer_size(encoder, data);
    return send_buffer(data, data_len);
}

