
#include "location.h"

#include <stdio.h>

/* This is a unique identifier for the message type. It can be any number you want it to be. 
 */
#define LOCATION_FIX_MSG_TYPE  1

/* Stack space for the location thread.
 */
char location_thread_stack[THREAD_STACKSIZE_DEFAULT];

/* Create the location thread 
 */
kernel_pid_t create_location_thread(kernel_pid_t receiver_pid, uint16_t message_type_id)
{
    location_t          location;

    location.pid = receiver_pid;
    location.type = message_type_id;

    kernel_pid_t location_pid = thread_create(location_thread_stack, sizeof(location_thread_stack),
                                                THREAD_PRIORITY_MAIN - 1, CREATE_STACKTEST,
                                                location_proc, &location, "location");
    return location_pid;
}

/* compare gps fix to msg.log
 */
void process_gpsfix_msg_content(location_gps_fix_t* gpsfix) {
 
/* TO DO set up compare to msg.log
 */
    printf("testing... %d\n", gpsfix->dimensions);
 }

/* Allocate a message queue for this thread. 
 */
msg_t msg_queue[LOCATION_MSG_Q_SIZE];

/* Main thread...
 */
int location_test(void) {
    msg_t           msg;
    int             msg_rc;
    kernel_pid_t    location_pid;
 
	/* Initialize the message queue for this thread 
     */
	msg_init_queue(msg_queue, LOCATION_MSG_Q_SIZE);

    /* Create the location thread.
     */    
    location_pid = create_location_thread(thread_getpid(), LOCATION_FIX_MSG_TYPE);


/* TO DO: Set up this loop so it walks the msg.log file and compares each message
 * in the log file to each received message. gpsfake should be launched with '-l'
 * in the command line so that it processes the file only once.
 *
 * Other tests that could be added would be to launch gpsfake with '-c 0.0' to 
 * check for message loss when the messages come real fast and with '-c 10.0' or
 * other very large number.
 */

    while (1) {
        /* Get a msg from the queue. NOTE change to blocking version.
         */
 printf("Getting msg...");
        msg_rc = msg_receive(&msg);
 printf("get is done...msg_rc = %d...", msg_rc);
        if (msg_rc == 1)
        {
 printf("processing\n");
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
        }
    }

	return 0;
}
