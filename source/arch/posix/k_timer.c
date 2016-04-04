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

}
