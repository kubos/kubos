#include "kubos-core/arch/kc_timer.h"

uint32_t kc_timer_now(void)
{
    return time(0);
}

void kc_timer_now_time(struct timeval * t)
{
    gettimeofday(t, NULL);
}


void kc_timer_usleep_until(uint32_t *last_wakeup, uint32_t usecs)
{

}
