#ifndef DAEMON_H
#define DAEMON_H
#include <csp/csp.h>
#include <command-and-control/types.h>
#include <tinycbor/cbor.h>

typedef int (*lib_function)(int, char**);

bool get_command(csp_socket_t* sock, char * command);
bool parse (char * args, cnc_command_wrapper * my_arguments);
void send_usage_error(cnc_command_wrapper * command);
bool start_encode_response(int message_type, cnc_command_wrapper * wrapper);
bool parse_command_cbor(csp_packet_t * packet, char * command);
bool encode_response(uint8_t * data, cnc_command_wrapper * wrapper, CborEncoder * encoder, CborEncoder * container);
bool finish_encode_response_and_send(uint8_t * data, CborEncoder *encoder, CborEncoder * container);
bool encode_processing_error(uint8_t * data, cnc_command_wrapper * result, CborEncoder * encoder, CborEncoder * container);
bool send_response(uint8_t * data, size_t data_len);
bool send_result(cnc_command_wrapper * wrapper);
#endif
