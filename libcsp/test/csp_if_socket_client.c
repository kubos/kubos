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

static bool server_running = false;

CSP_DEFINE_TASK(server_task) {
	csp_socket_t * socket = NULL;
	csp_conn_t * conn = NULL;
	csp_iface_t csp_socket_if;
	csp_socket_handle_t socket_driver;

	csp_socket_t * ext_socket = NULL;

	server_running = true;
	socket_init(&socket_driver, CSP_SOCKET_SERVER, TEST_SOCKET_PORT);

	csp_socket_init(&csp_socket_if, &socket_driver);
	csp_route_set(TEST_ADDRESS, &csp_socket_if, CSP_NODE_MAC);

	ext_socket = csp_socket(CSP_SO_NONE);

	csp_bind(ext_socket, TEST_EXT_PORT);
	csp_listen(ext_socket, 20);

	conn = csp_accept(ext_socket, 500);

	csp_route_end_task();
}

static void test_client(void ** arg) {
	csp_conn_t * conn = NULL;
	csp_socket_handle_t socket_driver;
	csp_iface_t csp_socket_if;
	csp_thread_handle_t server_task_handle;

	csp_buffer_init(20, 256);

	/* Init CSP with address MY_ADDRESS */
	csp_init(TEST_ADDRESS);

	/* Start router task with 500 word stack, OS task priority 1 */
	csp_route_start_task(500, 1);

	csp_thread_create(server_task, "SERVER", 1024, NULL, 0, &server_task_handle);

	while (!server_running) {
		csp_sleep_ms(100);
	}

	assert_int_equal(socket_init(&socket_driver, CSP_SOCKET_CLIENT, TEST_SOCKET_PORT), CSP_ERR_NONE);
	assert_int_equal(csp_socket_init(&csp_socket_if, &socket_driver), CSP_ERR_NONE);

	csp_route_set(TEST_ADDRESS, &csp_socket_if, CSP_NODE_MAC);
	conn = csp_connect(CSP_PRIO_NORM, TEST_ADDRESS, TEST_EXT_PORT, 1000, CSP_O_NONE);

	assert_non_null(conn);

	if (conn != NULL) {
		csp_packet_t * csp_packet = csp_buffer_get(strlen(msg));
		assert_non_null(csp_packet);
		memcpy(csp_packet->data, msg, strlen(msg));
		csp_packet->length = strlen(msg);
		assert_int_equal(csp_send(conn, csp_packet, 1000), 1);
	}

	csp_thread_kill(server_task_handle);

	return CSP_TASK_RETURN;
}

int main(void) {
	const struct CMUnitTest tests[] = {
		cmocka_unit_test(test_client),
	};

	return cmocka_run_group_tests(tests, NULL, NULL);
}