
#include "gps.h"
#include "xtimer.h"
#include <stdio.h>

#define ENABLE_DEBUG (1)
#include "debug.h"

/* This is a unique identifier for the message type. It can be any number you want it to be. 
 */
#define GPS_FIX_MSG_TYPE  1
#define GPS_UART 4

/* Stack space for the location thread.
 */
#define GPS_THREAD_SIZE THREAD_STACKSIZE_DEFAULT

char gps_thread_stack[GPS_THREAD_SIZE];

/* Do something with the gps data. Here we just print it.
 */
void process_gpsfix_msg_content(gps_fix_t* gpsfix) {
    /* size of buffer for printing timestamp 
     */
    #define BUFF_SIZE 32
    char    time_string[BUFF_SIZE];

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

/* Allocate a message queue for this thread. 
 */
msg_t msg_queue[GPS_MSG_Q_SIZE];

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

    // Initialize the message queue for this thread
    msg_init_queue(msg_queue, GPS_MSG_Q_SIZE);

    // Create the location thread.
    gps_cfg_t gps_cfg;
    gps_cfg.pid = thread_getpid();
    gps_cfg.type = GPS_FIX_MSG_TYPE;
    gps_cfg.uart = GPS_UART;
    gps_cfg.baudrate = 9600;
    gps_connect(&gps_cfg);

    while (1) {
        // Get a msg from the queue. The blocking version is msg_receive(&msg)
        if (msg_try_receive(&msg) != 1) {
            // Do whatever it is that you need to do. For demo purposes,
            // this thread will sleep for a while.
            xtimer_usleep_until(&last_wakeup, FAKE_PROCESSING_TIME);
            continue;
        }

        DEBUG("msg sender=%d, type=%d\n", msg.sender_pid, msg.type);
        switch (msg.type) {
            case GPS_FIX_MSG_TYPE:
                process_gpsfix_msg_content((gps_fix_t*) msg.content.ptr);
                break;
        }

    }

    return 0;
}
