#include <stdint.h>

/**
 * @brief A timex timestamp
 *
 * @note  If a timestamp is not normalized, the number of microseconds might be
 *        > 1000000
 */
typedef struct {
    uint32_t seconds;       /**< number of seconds */
    uint32_t microseconds;  /**< number of microseconds */
} timex_t;


static uint32_t kc_timer_now(void);

static void kc_timer_now_timex(timex_t * t);

void kc_timer_usleep_until(uint32_t *last_wakeup, uint32_t usecs);
