#pragma once

#include "command-and-control/types.h"
#include "tinycbor/cbor.h"


bool encode_packet(CborDataWrapper *data_wrapper, CNCCommandPacket * packet);
bool encode_command(CborDataWrapper * data_wrapper, CNCCommandPacket * packet, CborEncoder * encoder, CborEncoder * container);
bool start_encode_response(int message_type, CborDataWrapper * data_wrapper, CNCCommandPacket * packet);
bool finish_encode_response_and_send(CborDataWrapper * data_wrapper, CborEncoder *encoder, CborEncoder * container);
