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

#include <cmocka.h>
#include <csp/csp.h>
#include <csp/drivers/socket.h>

static void test_encode_packet(void ** arg) {
	csp_packet_t * packet;
	uint8_t buffer[256];

	csp_buffer_init(10, 200);

	packet = csp_buffer_get(100);

	assert_true(cbor_encode_csp_packet(packet, buffer));

	csp_buffer_free(packet);
}

static void test_parse_packet(void ** arg) {
	csp_packet_t *in_packet, *out_packet;
	uint8_t buffer[256];
	size_t buffer_size = 0;

	in_packet = csp_buffer_get(100);
	assert_non_null(in_packet);

	out_packet = csp_buffer_get(100);
	assert_non_null(out_packet);

	sprintf(in_packet->data, "test1234test");
	in_packet->length = strlen(in_packet->data);

	buffer_size = cbor_encode_csp_packet(in_packet, buffer);
	assert_true(buffer_size > 0);

	assert_true(cbor_parse_csp_packet(out_packet, buffer, buffer_size));

	assert_int_equal(in_packet->length, out_packet->length);
	assert_memory_equal(in_packet->data, out_packet->data, out_packet->length);

	csp_buffer_free(in_packet);
}

int main(void) {
	const struct CMUnitTest tests[] = {
		cmocka_unit_test(test_encode_packet),
		cmocka_unit_test(test_parse_packet)
	};

	return cmocka_run_group_tests(tests, NULL, NULL);
}