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

#include <csp/arch/csp_thread.h>
#include <csp/csp.h>
#include <csp/csp_error.h>
#include <csp/csp_interface.h>
#include <csp/drivers/socket.h>
#include <csp/interfaces/csp_if_socket.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdlib.h>

#include <tinycbor/cbor.h>

int cbor_encode_csp_packet(csp_packet_t * packet, uint8_t * buffer) {
	CborEncoder encoder, container;
	CborError err;

	if (buffer == NULL) {
		return -1;
	}
	cbor_encoder_init(&encoder, buffer, SOCKET_BUFFER_SIZE, 0);
	err = cbor_encoder_create_map(&encoder, &container, 3);
	if (err > 0) {
		return -err;
	}

	err = cbor_encode_text_stringz(&container, "LENGTH");
	if (err > 0) {
		return -err;
	}
	err = cbor_encode_half_float(&container, (void *)&(packet->length));
	if (err > 0) {
		return -err;
	}

	err = cbor_encode_text_stringz(&container, "ID");
	if (err > 0) {
		return -err;
	}
	err = cbor_encode_uint(&container, (packet->id.ext));
	if (err > 0) {
		return -err;
	}

	err = cbor_encode_text_stringz(&container, "DATA");
	if (err > 0) {
		return -err;
	}
	err = cbor_encode_text_string(&container, (char*)(packet->data), packet->length);
	if (err > 0) {
		return -err;
	}

	err = cbor_encoder_close_container_checked(&encoder, &container);
	if (err > 0) {
		return -err;
	}

	return cbor_encoder_get_buffer_size(&encoder, buffer);
}

bool cbor_parse_csp_packet(csp_packet_t * packet, void * buffer, int buffer_size) {
	CborParser parser;
	CborValue map, element;
	char * buf = NULL;

	if ((buffer == NULL) || (packet == NULL)) {
		return false;
	}

	CborError err = cbor_parser_init(buffer, buffer_size, 0, &parser, &map);
	if (err || !cbor_value_is_map(&map)) {
		return false;
	}
	err = cbor_value_map_find_value(&map, "LENGTH", &element);
	if (err) {
		return false;
	}

	if (cbor_value_get_half_float(&element, &(packet->length))) {
		return false;
	}

	err = cbor_value_map_find_value(&map, "ID", &element);
	if (err) {
		return false;
	}

	if (cbor_value_get_int(&element, (int*)&(packet->id.ext))) {
		return false;
	}

	err = cbor_value_map_find_value(&map, "DATA", &element);
	if (err) {
		return false;
	}

	if (!cbor_value_is_text_string(&element)) {
		csp_log_error("no text string\r\n");
		return false;
	}

	size_t byte_size;
	err = cbor_value_dup_text_string(&element, &buf, &byte_size, &element);
	if (err) {
		csp_log_error("Error parsing text string %d\r\n", err);
		free(buf);
		return false;
	} else {
		csp_log_info("Got that text string %d %d\r\n", byte_size, packet->length);
		memcpy((packet->data), buf, packet->length);
	}

	free(buf);

	return true;
}