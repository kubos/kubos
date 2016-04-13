/*
 * KubOS Core Flight Services
 * Copyright (C) 2016 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

 #ifdef KUBOS_CORE_POSIX

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

#endif
