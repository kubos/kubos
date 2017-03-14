#include <ipc/socket.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <telemetry-linux/server.h>
#include <telemetry/telemetry.h>

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

    while (running)
    {
        csp_sleep_ms(100);
    }

    csp_thread_kill(&thread_handle);

    telemetry_server_cleanup();

    return 0;
}
