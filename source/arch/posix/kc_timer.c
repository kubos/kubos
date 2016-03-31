#include "kubos-core/arch/kc_timer.h"

#include <time.h>

static uint32_t kc_timer_now(void)
{
    return time(0);
}

void kc_timer_usleep_until(uint32_t *last_wakeup, uint32_t usecs)
{

}
