#include <stdint.h>
#include <time.h>
#include <sys/time.h>


uint32_t k_timer_now(void);

void k_timer_now_time(struct timeval * t);

void k_timer_usleep_until(uint32_t *last_wakeup, uint32_t usecs);
