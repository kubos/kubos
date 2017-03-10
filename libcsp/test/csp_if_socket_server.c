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
#include <csp/interfaces/csp_if_socket.h>

#define TEST_INT_PORT 10
#define TEST_EXT_PORT 11
#define TEST_NUM_CON 5
#define TEST_ADDRESS 1
#define TEST_SOCKET_PORT 8189

static char msg[] = "test123test";

static bool test_running = true;

CSP_DEFINE_TASK(client_task) {
	csp_conn_t * conn = NULL;
	csp_socket_handle_t socket_driver;
	csp_iface_t csp_socket_if;

	while (test_running) {
		if (socket_init(&socket_driver, CSP_SOCKET_CLIENT, TEST_SOCKET_PORT) != CSP_ERR_NONE) {
			continue;
		}

		if (csp_socket_init(&csp_socket_if, &socket_driver)) {
			csp_socket_close(&csp_socket_if, &socket_driver);
			continue;
		}

		csp_route_set(TEST_ADDRESS, &csp_socket_if, CSP_NODE_MAC);
		if ((conn = csp_connect(CSP_PRIO_NORM, TEST_ADDRESS, TEST_EXT_PORT, 1000, CSP_O_NONE)) != NULL) {
			break;
		}
	}

	if (conn != NULL) {
		csp_packet_t * csp_packet = csp_buffer_get(strlen(msg));
		memcpy(csp_packet->data, msg, strlen(msg));
		csp_packet->length = strlen(msg);
		if (!csp_send(conn, csp_packet, 1000))
			csp_buffer_free(csp_packet);
	}

	csp_thread_exit();
}

static void test_server(void ** arg) {
	csp_conn_t * conn = NULL;
	csp_iface_t csp_socket_if = {
		.next = NULL
	};
	csp_socket_handle_t socket_driver;
	csp_thread_handle_t client_task_handle;
	csp_socket_t * ext_socket = NULL;
	csp_packet_t * packet = NULL;
	char buffer[100];

	csp_buffer_init(20, 256);

	/* Init CSP with address MY_ADDRESS */
	assert_int_equal(csp_init(TEST_ADDRESS), CSP_ERR_NONE);

	/* Start router task with 500 word stack, OS task priority 1 */
	csp_route_start_task(500, 1);

	csp_thread_create(client_task, "CLIENT", 1024, NULL, 0, &client_task_handle);

	assert_int_equal(socket_init(&socket_driver, CSP_SOCKET_SERVER, TEST_SOCKET_PORT), CSP_ERR_NONE);

	assert_int_equal(csp_socket_init(&csp_socket_if, &socket_driver), CSP_ERR_NONE);
	csp_route_set(TEST_ADDRESS, &csp_socket_if, CSP_NODE_MAC);

	ext_socket = csp_socket(CSP_SO_NONE);
	assert_non_null(ext_socket);
	assert_int_equal(csp_bind(ext_socket, TEST_EXT_PORT), CSP_ERR_NONE);
	assert_int_equal(csp_listen(ext_socket, 20), CSP_ERR_NONE);

	conn = csp_accept(ext_socket, 500);
	assert_non_null(conn);

	packet = csp_read(conn, 1000);
	assert_non_null(packet);
	memcpy(buffer, (void *)packet->data, packet->length);
	assert_string_equal(buffer, msg);
	csp_buffer_free(packet);

	test_running = false;

	csp_close_socket(ext_socket);

	csp_thread_kill(client_task_handle);

	csp_socket_close(&csp_socket_if, &socket_driver);

	csp_route_end_task();

	csp_terminate();

	csp_buffer_cleanup();
}

int main(void) {
	const struct CMUnitTest tests[] = {
		cmocka_unit_test(test_server),
	};

	return cmocka_run_group_tests(tests, NULL, NULL);
}