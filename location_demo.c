
#include "location.h"
#include "xtimer.h"

#include <stdio.h>

/* This is a unique identifier for the message type. It can be any number you want it to be. 
 */
#define LOCATION_FIX_MSG_TYPE  1

/* Stack space for the location thread.
 */
char location_thread_stack[THREAD_STACKSIZE_DEFAULT];

/* Do something with the gps data. Here we just print it.
 */
void process_gpsfix_msg_content(location_gps_fix_t* gpsfix) {
    /* size of buffer for printing timestamp 
     */
    #define BUFF_SIZE 32
    char    time_string[BUFF_SIZE];

    /* Convert unix+milliseconds time to a string.
     */
    struct tm* local = localtime(&gpsfix->time);
    sprintf(time_string, "%02d:%02d:%02d.%03d", local->tm_hour, local->tm_min, local->tm_sec, gpsfix->milliseconds);

    /* A 3D fix has good Altitude and Climb info in it
     */
    if (gpsfix->dimensions == LOC_DIM_3D) {
        printf("3D Fix time: %s\n"
               "     Latitude:  %g\n"
               "     Longitude: %g\n"
               "     Altitude:  %g\n"
               "     Speed:     %g\n"
               "     Climb:     %g\n\n", time_string,
               gpsfix->latitude, gpsfix->longitude, gpsfix->altitude,
               gpsfix->speed, gpsfix->climb);
    } else {
        /* A 2D fix does not have good Altitude and Climb info in it
         */
        printf("2D Fix time: %s\n"
               "     Latitude:  %g\n"
               "     Longitude: %g\n"
               "     Altitude:  unknown\n"
               "     Speed:     %g\n"
               "     Climb:     unknown\n\n", time_string,
               gpsfix->latitude, gpsfix->longitude, gpsfix->speed);
    }
}

/* Allocate a message queue for this thread. 
 */
msg_t msg_queue[LOCATION_MSG_Q_SIZE];

/* This is how much time the demo program will spend doing
 * work other than location related work. 0.75 seconds
 */
#define FAKE_PROCESSING_TIME (750000U)

/* Main thread...
 */
int location_demo(int argc, char **argv) {
    /* Suppress compiler errors */
    (void)argc;
    (void)argv;
    msg_t           msg;
    uint32_t        last_wakeup = xtimer_now();

	/* Initialize the message queue for this thread 
     */
	msg_init_queue(msg_queue, LOCATION_MSG_Q_SIZE);

    // Create the location thread.
    gps_cfg_t gps_cfg;
    gps_cfg.pid = thread_getpid();
    gps_cfg.type = GPS_FIX_MSG_TYPE;
    gps_cfg.uart = GPS_UART;
    gps_cfg.baudrate = 9600;
    gps_connect(&gps_cfg);

    /* Loop until you get a CTRL-C
     */
    while (1) {
        /* Get a msg from the queue. The blocking version is msg_receive(&msg).
         */
        if (msg_try_receive(&msg) == 1)
        {
            /* Process the message...
             */
            if (msg.sender_pid == location_pid) {
            	/* this message came from location
                 */
            	if (msg.type == LOCATION_FIX_MSG_TYPE) {
            		/* This is a gps fix type message
                     */
                    process_gpsfix_msg_content((location_gps_fix_t*)msg.content.ptr);
                }
        	}
        } else {
            /* Do whatever it is that you need to do. For demo purposes,
             * this thread will sleep for a while.
             */
            xtimer_usleep_until(&last_wakeup, FAKE_PROCESSING_TIME);
        }

    }

	return 0;
}
