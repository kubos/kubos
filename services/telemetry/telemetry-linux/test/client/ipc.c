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

#include <ipc/pubsub_socket.h>
#include <cmocka.h>

bool __wrap_kprv_socket_client_connect(socket_conn * conn, uint8_t port)
{
    if (conn != NULL)
    {
        conn->socket_handle = 10;
        conn->is_active = true;
    }
    return mock_type(bool);
}

bool __wrap_kprv_socket_send(socket_conn * conn, void * data, uint16_t length)
{
    check_expected(conn->is_active);
    check_expected(data);
    
    return mock_type(bool);
}

bool __wrap_kprv_socket_recv(socket_conn * conn, void * buffer, int buffer_size, uint32_t * size_read)
{
    check_expected(conn->is_active);
    check_expected(buffer);
    buffer = mock_type(void*);
    return mock_type(bool);
}

bool __wrap_kprv_socket_close(socket_conn * conn)
{
    conn->is_active = false;
    return mock_type(bool);
}