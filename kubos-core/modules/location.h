#include "time.h"
//#include "thread.h"

#include "kubos-core/arch/kc_msg.h"
#include "kubos-core/arch/kc_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct location{
	kernel_pid_t pid;
	uint16_t type;
} location_t;

/* libgps returns the timestamp as a double. The Unix time_t is platform dependent
 * and has no provision for milliseconds so location breaks the timestamp into a
 * platform specific time_t plus milliseconds.
 */

typedef struct {
	time_t 	time;			/* Unix time in seconds */
	int		milliseconds;
	int	   	dimensions;		/* how many dimensions for this fix */
#define LOC_DIM_2D  	2	/* good for latitude/longitude */
#define LOC_DIM_3D  	3	/* good for altitude/climb too */
    double 	latitude;		/* Latitude in degrees */
    double 	longitude;		/* Longitude in degrees */
    double 	altitude;		/* Altitude in meters */
    double 	speed;			/* Speed over ground, meters/sec */
    double 	climb;       	/* Vertical speed, meters/sec */
} location_gps_fix_t;

#define LOCATION_MSG_Q_SIZE 32 /* Must be a power of 2! */

extern void *location_proc(void* config);

/*
#ifdef LOCATION_INTERNAL
location_gps_fix_t* get_gps_fix(void);
double trunc(double num);
double round(double num);
void fixup_timestamp(double dtime, time_t* time, int* milliseconds);
#endif
*/

#ifdef __cplusplus
}
#endif
