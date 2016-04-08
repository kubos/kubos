#include "kubos-core/arch/k_timer.h"

uint32_t k_timer_now(void)
{
    return time(0);
}

void k_timer_now_time(struct timeval * t)
{
    gettimeofday(t, NULL);
}


void k_timer_usleep_until(uint32_t *last_wakeup, uint32_t usecs)
{
    struct timespec t;
    t.tv_sec = usecs / 1000000;
    t.tv_nsec = (usecs % 1000000) * 1000;
    nanosleep(&t, NULL);
}
