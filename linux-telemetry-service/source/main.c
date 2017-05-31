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
 
#include <ipc/socket.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <telemetry-linux/server.h>
#include <telemetry/telemetry.h>

#include "publishers.h"

static bool running = true;
static socket_conn server_conn;

void terminate(int signum)
{
    printf("Caught term..time 2 go\r\n");
    running = false;
}

CSP_DEFINE_TASK(main_thread)
{
    printf("Telemetry service beginning..\r\n");

    while (running)
    {
        printf("Waiting on new telemetry client...\r\n");
        socket_conn conn;
        while (!kprv_socket_server_accept(&server_conn, &conn))
        {
            continue;
        }

        subscriber_list_item * sub = kprv_subscriber_init(conn);
        if (sub != NULL)
        {
            csp_thread_create(client_handler, NULL, 1000, sub, 0, &(sub->rx_thread));
            kprv_subscriber_add(sub);
        }
    }

    printf("Server shut down...\r\n");
}

int main(int argc, char ** argv)
{
    csp_thread_handle_t thread_handle;
    struct sigaction action = { 0 };

    kprv_socket_server_setup(&server_conn, TELEMETRY_SOCKET_PORT, TELEMETRY_SUBSCRIBERS_MAX_NUM);

    action.sa_handler = terminate;
    sigaction(SIGTERM, &action, NULL);

    csp_thread_create(main_thread, NULL, 1000, NULL, 0, &thread_handle);

    #ifdef TARGET_LIKE_ISIS
    csp_thread_handle_t supervisor_handle;
    csp_thread_create(supervisor_publisher, NULL, 1000, NULL, 0, &supervisor_handle);
    #endif

    while (running)
    {
        csp_sleep_ms(100);
    }

    csp_thread_kill(&thread_handle);
    
    #ifdef TARGET_LIKE_ISIS
    csp_thread_kill(&supervisor_handle);
    #endif

    telemetry_server_cleanup();

    return 0;
}
