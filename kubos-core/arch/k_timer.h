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


 /**
  *
  * @file       k_timer.h
  * @brief      Timer interface
  *
  * @author     kubos.co
  */


#include <stdint.h>
#include <time.h>
#include <sys/time.h>


uint32_t k_timer_now(void);

void k_timer_now_time(struct timeval * t);

void k_timer_usleep_until(uint32_t *last_wakeup, uint32_t usecs);
