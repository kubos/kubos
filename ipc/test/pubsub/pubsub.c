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
#include <cmocka.h>
#include "ipc/pubsub.h"

#define TEST_ADDRESS 11
#define TEST_PORT 12

static void test_server_setup(void ** arg)
{
    csp_socket_t * socket = NULL;

    will_return(__wrap_csp_socket, "");

    expect_not_value(__wrap_csp_bind, socket, NULL);
    will_return(__wrap_csp_bind, CSP_ERR_NONE);

    expect_not_value(__wrap_csp_listen, socket, NULL);
    will_return(__wrap_csp_listen, CSP_ERR_NONE);
    
    assert_true(socket = kprv_server_setup(TEST_PORT, 1));
}

static void test_server_accept_null_socket(void ** arg)
{
    pubsub_conn conn;
    assert_false(kprv_server_accept(NULL, &conn));
}

static void test_server_accept_null_conn(void ** arg)
{
    csp_socket_t * socket = NULL;
    
    will_return(__wrap_csp_socket, "");

    expect_not_value(__wrap_csp_bind, socket, NULL);
    will_return(__wrap_csp_bind, CSP_ERR_NONE);

    expect_not_value(__wrap_csp_listen, socket, NULL);
    will_return(__wrap_csp_listen, CSP_ERR_NONE);

    socket = kprv_server_setup(TEST_PORT, 1);

    assert_false(kprv_server_accept(socket, NULL));
}

static void test_server_accept(void ** arg)
{
    pubsub_conn conn;
    csp_socket_t * socket = NULL;

    will_return(__wrap_csp_socket, "");

    expect_not_value(__wrap_csp_bind, socket, NULL);
    will_return(__wrap_csp_bind, CSP_ERR_NONE);

    expect_not_value(__wrap_csp_listen, socket, NULL);
    will_return(__wrap_csp_listen, CSP_ERR_NONE);

    socket = kprv_server_setup(TEST_PORT, 1);

    expect_value(__wrap_csp_accept, socket, socket);
    will_return(__wrap_csp_accept, "");

    assert_true(kprv_server_accept(socket, &conn));
}

static void test_subscriber_connect_null_conn(void ** arg)
{
    assert_false(kprv_subscriber_connect(NULL, TEST_ADDRESS, TEST_PORT));
}

static void test_subscriber_connect(void ** arg)
{
    pubsub_conn conn;

    will_return(__wrap_csp_connect, "");

    assert_true(kprv_subscriber_connect(&conn, TEST_ADDRESS, TEST_PORT));
    assert_true(conn.conn_handle != NULL);
}

static void test_send_null_data(void ** arg)
{
    pubsub_conn conn;
    assert_false(kprv_send_csp(conn, NULL, 0));
}

static void test_send_bad_length(void ** arg)
{
    pubsub_conn conn;
    int data = 10;
    assert_false(kprv_send_csp(conn, &data, -1));
}

static void test_send_null_conn_handle(void ** arg)
{
    pubsub_conn conn;
    int data = 10;
    assert_false(kprv_send_csp(conn, &data, sizeof(int)));
}

static void test_send(void ** arg)
{
    pubsub_conn conn;
    char data = 'A';
    csp_socket_t * socket = NULL;

    will_return(__wrap_csp_socket, "");

    expect_not_value(__wrap_csp_bind, socket, NULL);
    will_return(__wrap_csp_bind, CSP_ERR_NONE);

    expect_not_value(__wrap_csp_listen, socket, NULL);
    will_return(__wrap_csp_listen, CSP_ERR_NONE);

    socket = kprv_server_setup(TEST_PORT, 1);
    
    expect_value(__wrap_csp_accept, socket, socket);
    will_return(__wrap_csp_accept, "");
    
    kprv_server_accept(socket, &conn);

    expect_value(__wrap_csp_send, conn, conn.conn_handle);
    expect_not_value(__wrap_csp_send, packet, NULL);
    will_return(__wrap_csp_send, 1);

    assert_true(kprv_send_csp(conn, (void*)&data, sizeof(data)));
}

static void test_publisher_read_null_conn(void ** arg)
{
    pubsub_conn conn;
    conn.conn_handle = NULL;
    int data = 10;
    assert_false(kprv_publisher_read(conn, &data, 1, TEST_PORT));
}

static void test_publisher_read_null_buffer(void ** arg)
{
    pubsub_conn conn;
    csp_socket_t * socket = NULL;
    
    will_return(__wrap_csp_socket, "");

    expect_not_value(__wrap_csp_bind, socket, NULL);
    will_return(__wrap_csp_bind, CSP_ERR_NONE);

    expect_not_value(__wrap_csp_listen, socket, NULL);
    will_return(__wrap_csp_listen, CSP_ERR_NONE);

    socket = kprv_server_setup(TEST_PORT, 1);
    
    expect_value(__wrap_csp_accept, socket, socket);
    will_return(__wrap_csp_accept, "");
    
    kprv_server_accept(socket, &conn);

    assert_false(kprv_publisher_read(conn, NULL, 1, TEST_PORT));
}

static void test_publisher_read(void ** arg)
{
    pubsub_conn conn;
    char buffer;
    csp_socket_t * socket = NULL;
    
    will_return(__wrap_csp_socket, "");

    expect_not_value(__wrap_csp_bind, socket, NULL);
    will_return(__wrap_csp_bind, CSP_ERR_NONE);

    expect_not_value(__wrap_csp_listen, socket, NULL);
    will_return(__wrap_csp_listen, CSP_ERR_NONE);

    socket = kprv_server_setup(TEST_PORT, 1);
    
    expect_value(__wrap_csp_accept, socket, socket);
    will_return(__wrap_csp_accept, "");
    
    kprv_server_accept(socket, &conn);

    expect_value(__wrap_csp_read, conn, conn.conn_handle);

    will_return(__wrap_csp_conn_dport, TEST_PORT);

    assert_true(kprv_publisher_read(conn, &buffer, 1, TEST_PORT));
}

static void test_subscriber_read_null_conn(void ** arg)
{
    pubsub_conn conn;
    conn.conn_handle = NULL;
    int data = 10;
    assert_false(kprv_subscriber_read(conn, &data, 1, TEST_PORT));
}

static void test_subscriber_read_null_buffer(void ** arg)
{
    pubsub_conn conn;

    will_return(__wrap_csp_connect, "");

    kprv_subscriber_connect(&conn, TEST_ADDRESS, TEST_PORT);
    
    assert_false(kprv_subscriber_read(conn, NULL, 1, TEST_PORT));
}

static void test_subscriber_read(void ** arg)
{
    pubsub_conn conn;
    char buffer;

    will_return(__wrap_csp_connect, "");

    kprv_subscriber_connect(&conn, TEST_ADDRESS, TEST_PORT);

    expect_value(__wrap_csp_read, conn, conn.conn_handle);
    will_return(__wrap_csp_conn_sport, TEST_PORT);
    
    assert_true(kprv_subscriber_read(conn, &buffer, 1, TEST_PORT));
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_server_setup),
        cmocka_unit_test(test_server_accept_null_socket),
        cmocka_unit_test(test_server_accept_null_conn),
        cmocka_unit_test(test_server_accept),
        cmocka_unit_test(test_subscriber_connect_null_conn),
        cmocka_unit_test(test_subscriber_connect),
        cmocka_unit_test(test_send_null_data),
        cmocka_unit_test(test_send_bad_length),
        cmocka_unit_test(test_send_null_conn_handle),
        cmocka_unit_test(test_send),
        cmocka_unit_test(test_publisher_read_null_conn),
        cmocka_unit_test(test_publisher_read_null_buffer),
        cmocka_unit_test(test_publisher_read),
        cmocka_unit_test(test_subscriber_read_null_conn),
        cmocka_unit_test(test_subscriber_read_null_buffer),
        cmocka_unit_test(test_subscriber_read),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}