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

#include <ipc/pubsub.h>
#include <cmocka.h>

bool __wrap_kprv_subscriber_socket_connect(pubsub_conn * conn, uint8_t address, uint8_t port)
{
    if (conn != NULL)
    {
        conn->conn_handle = mock_type(csp_conn_t *);
    }
    return mock_type(bool);
}

bool __wrap_kprv_send_csp(pubsub_conn * conn, void * data, uint16_t length)
{
    check_expected(conn->conn_handle);
    check_expected(data);
    
    return mock_type(bool);
}

int __wrap_csp_close(csp_conn_t * conn)
{
    check_expected(conn);
    return mock_type(int);
}

bool __wrap_kprv_subscriber_read(pubsub_conn * conn, void * buffer, int buffer_size, uint8_t port)
{
    check_expected(conn->conn_handle);
    check_expected(buffer);

    return mock_type(bool);
}

void __wrap_kprv_subscriber_socket_close(pubsub_conn * conn)
{
    check_expected(conn);
    conn->conn_handle = NULL;
}