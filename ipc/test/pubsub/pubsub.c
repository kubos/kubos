/*
 * KubOS HAL
 * Copyright (C) 2016 Kubos Corporation
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
#include <unity/unity.h>
#include <unity/k_test.h>
#include "ipc/pubsub.h"

#define TEST_ADDRESS 11
#define TEST_PORT 12


static void test_server_setup_null_socket(void)
{
    TEST_ASSERT_EQUAL_INT(server_setup(NULL, TEST_PORT, 1), false);
}

static void test_server_setup(void)
{
    csp_socket_t * socket = NULL;
    
    TEST_ASSERT_EQUAL_INT(server_setup(&socket, TEST_PORT, 1), true);

    csp_buffer_free(socket);
}

static void test_server_accept_null_socket(void)
{
    pubsub_conn conn;
    TEST_ASSERT_EQUAL_INT(server_accept(NULL, &conn), false);
}

static void test_server_accept_null_conn(void)
{
    csp_socket_t * socket = NULL;
    
    server_setup(&socket, TEST_PORT, 1);

    TEST_ASSERT_EQUAL_INT(server_accept(&socket, NULL), false);

    csp_buffer_free(socket);
}

static void test_server_accept(void)
{
    pubsub_conn conn;
    csp_socket_t * socket = NULL;
    server_setup(&socket, TEST_PORT, 1);

    TEST_ASSERT_EQUAL_INT(server_accept(&socket, &conn), true);

    csp_buffer_free(conn.conn_handle);
    csp_buffer_free(socket);
}

static void test_subscriber_connect_null_conn(void)
{
    TEST_ASSERT_EQUAL_INT(subscriber_connect(NULL, TEST_ADDRESS, TEST_PORT), false);
}

static void test_subscriber_connect(void)
{
    pubsub_conn conn;

    TEST_ASSERT_EQUAL_INT(subscriber_connect(&conn, TEST_ADDRESS, TEST_PORT), true);
    TEST_ASSERT_EQUAL_INT(true, (conn.conn_handle != NULL));

    csp_buffer_free(conn.conn_handle);
}

static void test_send_null_data(void)
{
    pubsub_conn conn;
    TEST_ASSERT_EQUAL_INT(send_csp(conn, NULL, 0), false);
}

static void test_send_bad_length(void)
{
    pubsub_conn conn;
    int data = 10;
    TEST_ASSERT_EQUAL_INT(send_csp(conn, &data, -1), false);
}

static void test_send_null_conn_handle(void)
{
    pubsub_conn conn;
    int data = 10;
    TEST_ASSERT_EQUAL_INT(send_csp(conn, &data, sizeof(int)), false);
}

static void test_send(void)
{
    pubsub_conn conn;
    int data = 10;
    csp_socket_t * socket = NULL;
    
    server_setup(&socket, TEST_PORT, 1);
    server_accept(&socket, &conn);

    TEST_ASSERT_EQUAL_INT(true, send_csp(conn, (void*)&data, sizeof(data)));

    csp_buffer_free(conn.conn_handle);
    csp_buffer_free(socket);
}

static void test_publisher_read_null_conn(void)
{
    pubsub_conn conn;
    conn.conn_handle = NULL;
    int data = 10;
    TEST_ASSERT_EQUAL_INT(publisher_read(conn, &data, 1, TEST_PORT), false);
}

static void test_publisher_read_null_buffer(void)
{
    pubsub_conn conn;
    csp_socket_t * socket = NULL;
    
    server_setup(&socket, TEST_PORT, 1);
    server_accept(&socket, &conn);

    TEST_ASSERT_EQUAL_INT(publisher_read(conn, NULL, 1, TEST_PORT), false);

    csp_buffer_free(conn.conn_handle);
    csp_buffer_free(socket);
}

static void test_publisher_read(void)
{
    pubsub_conn conn;
    char buffer;
    csp_socket_t * socket = NULL;
    
    server_setup(&socket, TEST_PORT, 1);
    server_accept(&socket, &conn);

    TEST_ASSERT_EQUAL_INT(true, publisher_read(conn, &buffer, 1, TEST_PORT));

    csp_buffer_free(conn.conn_handle);
    csp_buffer_free(socket);
}

static void test_subscriber_read_null_conn(void)
{
    pubsub_conn conn;
    conn.conn_handle = NULL;
    int data = 10;
    TEST_ASSERT_EQUAL_INT(subscriber_read(conn, &data, 1, TEST_PORT), false);
}

static void test_subscriber_read_null_buffer(void)
{
    pubsub_conn conn;

    subscriber_connect(&conn, TEST_ADDRESS, TEST_PORT);
    
    TEST_ASSERT_EQUAL_INT(subscriber_read(conn, NULL, 1, TEST_PORT), false);

    csp_buffer_free(conn.conn_handle);
}

static void test_subscriber_read(void)
{
    pubsub_conn conn;
    char buffer;

    subscriber_connect(&conn, TEST_ADDRESS, TEST_PORT);
    
    TEST_ASSERT_EQUAL_INT(subscriber_read(conn, &buffer, 1, TEST_PORT), true);

    csp_buffer_free(conn.conn_handle);
}

K_TEST_MAIN()
{
    UNITY_BEGIN();
    
    RUN_TEST(test_publisher_read_null_conn);
    RUN_TEST(test_publisher_read_null_buffer);
    RUN_TEST(test_publisher_read);
    
    RUN_TEST(test_subscriber_read_null_conn);
    RUN_TEST(test_subscriber_read_null_buffer);
    RUN_TEST(test_subscriber_read);

    RUN_TEST(test_send_null_data);
    RUN_TEST(test_send_bad_length);
    RUN_TEST(test_send_null_conn_handle);
    RUN_TEST(test_send);

    RUN_TEST(test_subscriber_connect_null_conn);
    RUN_TEST(test_subscriber_connect);

    RUN_TEST(test_server_accept_null_conn);
    RUN_TEST(test_server_accept_null_socket);
    RUN_TEST(test_server_accept);

    RUN_TEST(test_server_setup_null_socket);
    RUN_TEST(test_server_setup);
    
    return UNITY_END();
}

int main(void)
{
    K_TEST_RUN_MAIN();
}