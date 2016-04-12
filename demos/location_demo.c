
#include "kubos-core/arch/k_timer.h"
#include "kubos-core/modules/gps.h"
#include <stdio.h>
#include <stdlib.h>

#define ENABLE_DEBUG (1)
#include "kubos-core/k_debug.h"

/* This is a unique identifier for the message type. It can be any number you want it to be.
 */
#define GPS_FIX_MSG_TYPE  1
#define GPS_UART 4

#define MY_ADDRESS  1			// Address of local CSP node
#define MY_PORT		10			// Port to send test traffic to


/* Do something with the gps data. Here we just print it.
 */
void process_gpsfix_msg_content(gps_fix_t* gpsfix) {
    /* size of buffer for printing timestamp
     */
    #define BUFF_SIZE 32
    char    time_string[BUFF_SIZE];

    if (NULL != gpsfix)
    {

        sprintf(time_string, "%02d:%02d:%02d.%03d", gpsfix->hour, gpsfix->minute,
                gpsfix->seconds, gpsfix->milliseconds);

        printf("Fix Time: %s\n"
                "    Latitude:  %f\n"
                "    Longitude: %f\n"
                "    Altitude:  %f\n"
                "    Speed:     %f\n"
                "    Climb:     %f\n\n", time_string,
                gpsfix->latitude, gpsfix->longitude, gpsfix->altitude,
                gpsfix->speed, gpsfix->climb);
    }
    else
    {
        DEBUG("received NULL gpsfix\n");
    }
}

/* This is how much time the demo program will spend doing
 * work other than location related work. 0.75 seconds
 */
#define FAKE_PROCESSING_TIME (750000U)

/* Main thread...
 */
int main(int argc, char **argv) {
    /* Suppress compiler errors */
    (void)argc;
    (void)argv;
    csp_conn_t *conn;
	csp_packet_t *packet;
    uint32_t last_wakeup = k_timer_now();


    printf("Initialising CSP\r\n");
	csp_buffer_init(5, 300);

	/* Init CSP with address MY_ADDRESS */
	csp_init(MY_ADDRESS);

	/* Start router task with 500 word stack, OS task priority 1 */
	csp_route_start_task(500, 1);

    /* Create socket without any socket options */
	csp_socket_t *sock = csp_socket(CSP_SO_NONE);
	/* Bind all ports to socket */
	csp_bind(sock, CSP_ANY);
	/* Create 10 connections backlog queue */
	csp_listen(sock, 10);


	/* Pointer to current connection and packet */
	csp_conn_t * gps_conn;
    gps_conn = csp_connect(CSP_PRIO_NORM, MY_ADDRESS, MY_PORT, 1000, CSP_O_NONE);

    struct uart_conf * conf = malloc(sizeof(struct uart_conf));

    conf->device = argc != 2 ? "/dev/pts/5" : argv[1];
    conf->baudrate = 500000;

    // Create the location thread.
    gps_cfg_t gps_cfg;
    gps_cfg.conn = gps_conn;
    gps_cfg.type = GPS_FIX_MSG_TYPE;
    gps_cfg.uart_conf = conf;
    gps_connect(&gps_cfg);

    while (1) {
        /* Wait for connection, 10000 ms timeout */
		if ((conn = csp_accept(sock, 10000)) == NULL)
			continue;

        // Get a msg from csp
        if ((packet = csp_read(conn, 100)) == NULL) {
            printf("got null packet\n");
            // Do whatever it is that you need to do. For demo purposes,
            // this thread will sleep for a while.
            k_timer_usleep_until(&last_wakeup, FAKE_PROCESSING_TIME);
            continue;
        }

        printf("got packet\n");

        process_gpsfix_msg_content((gps_fix_t*)packet->data);

        csp_close(conn);

    }

    return 0;
}
