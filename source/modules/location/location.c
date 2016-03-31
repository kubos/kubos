/* #define LOCATION_INTERNAL */
#include "kubos-core/modules/location.h"

//#include "kernel.h"

#include "kubos-core/arch/kc_gps.h"
#include "kubos-core/arch/kc_timer.h"

/* time to sleep between attempts to connect 3 seconds*/
#define CONNECT_RETRY_INTERVAL (3000000U)

/* set polling interval to .5 second */
#define POLLING_INTERVAL (500000U)

static struct gps_data_t gpsdata;

/* Circular buffer of gps fixes to pass as message content */
static int next_buffer_slot = 0;
static location_gps_fix_t buffer[LOCATION_MSG_Q_SIZE];

location_gps_fix_t* get_gps_fix(void) {
	location_gps_fix_t* ptr = &buffer[next_buffer_slot];
	next_buffer_slot++;
	if (next_buffer_slot >= LOCATION_MSG_Q_SIZE) {
		next_buffer_slot = 0;
	}
	return ptr;
}

/*-------------------------------------------------------------------
 * The following functions belong in the math library, but I
 * don't want to take the time to work on that now.
 * CAUTION: won't handle NaN, infinity, etc.
 */
double trunc(double num) {
	return ((double)(int)num);
}

double round(double num) {
	int 	fraction;

	fraction = (int)((num - trunc(num)) * 10);

	return fraction >= 5 ? (trunc(num) + 1.0) : (fraction <= -5 ? trunc(num) - 1.0 : trunc(num));
}

/* End of math functions
 *-------------------------------------------------------------------
 */

/* Split milliseconds away from the unix time_t portion of the timestamp
 */
void fixup_timestamp(double dtime, time_t* time, int* milliseconds)
{
	double intpart = trunc(dtime);
	double fractpart = dtime - intpart;;

	*milliseconds = (int)round(fractpart * 1000.0);
	*time = (time_t)intpart;
}

/* Establish a connection with gpsd. Function blocks until there's a conncetion.
 */
void gpsconnect(void){
	bool 		not_connected = true;
	uint32_t	last_wakeup = kc_timer_now();

	while (not_connected) {
		// Open a socket connection to gpsd
		if (gps_open("127.0.0.1", DEFAULT_GPSD_PORT, &gpsdata) == 0) {
			//register for updates
			gps_stream(&gpsdata, WATCH_ENABLE | WATCH_NEWSTYLE, NULL);
			not_connected = false;
		} else {
			// Sleep for a while before trying again
			kc_timer_usleep_until(&last_wakeup, CONNECT_RETRY_INTERVAL);
		}
	}
}


/* This is the main location function. This function reads from gpsd and
 * then passes gps fix messages to it's parent thread.
 */
void *location_proc(void* config) {
	msg_t 		    	gpsmsg;
	location_gps_fix_t* gpsfix;
	kernel_pid_t 		consumer_pid = ((location_t *)config)->pid;
	uint16_t 			msg_type = ((location_t *)config)->type;
	uint32_t 			last_wakeup = kc_timer_now();
	int 				bytes_read;

	gpsconnect();

	while (1) {
		gpsdata.set = (gps_mask_t)0ULL;
		bytes_read = gps_read(&gpsdata);

		if (bytes_read == -1) {
			// -1 indicates an error so assume no connection to gpsd.
			gpsconnect();
		} else if (bytes_read > 0){
			//sometimes if your GPS doesn't have a fix, it sends you data anyways
			//the values for the fix are NaN. this is a clever way to check for NaN.
			if (gpsdata.fix.longitude == gpsdata.fix.longitude
					&& gpsdata.fix.altitude == gpsdata.fix.altitude
					&& gpsdata.fix.mode >= MODE_2D) {
				// you have a legitimate fix!
				// Send data to queue
				gpsfix = get_gps_fix();
				fixup_timestamp(gpsdata.fix.time, &gpsfix->time, &gpsfix->milliseconds);

				gpsfix->dimensions = gpsdata.fix.mode;
				gpsfix->latitude = gpsdata.fix.latitude;
				gpsfix->longitude = gpsdata.fix.longitude;
				gpsfix->altitude = gpsdata.fix.altitude;
				gpsfix->speed = gpsdata.fix.speed;
				gpsfix->climb = gpsdata.fix.climb;

				gpsmsg.type = msg_type;
				gpsmsg.content.ptr = (char*)gpsfix;
				kc_msg_send(&gpsmsg, consumer_pid);
			}
		} else {
			kc_timer_usleep_until(&last_wakeup, POLLING_INTERVAL);
		}
	}

	//cleanup...this will never happen...
	gps_stream(&gpsdata, WATCH_DISABLE, NULL);
	gps_close(&gpsdata);

	return NULL;
}
