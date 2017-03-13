#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include <ipc/socket.h>

#include <telemetry-linux/server.h>
#include <telemetry/telemetry.h>

int main(int argc, char ** argv)
{
    socket_conn server_conn;
    kprv_socket_server_setup(&server_conn, TELEMETRY_SOCKET_PORT, TELEMETRY_SUBSCRIBERS_MAX_NUM);
    bool running = true;

    while (running)
    {
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

    telemetry_server_cleanup();

    return 0;
}
