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

#ifndef K_TEST_H
#define K_TEST_H

#ifdef TARGET_LIKE_FREERTOS
#include "FreeRTOS.h"
#include "task.h"
#include "kubos-hal/uart.h"

#define K_TEST_MAIN() \
static int __test_result = -1; \
static int __test_main (void); \
static void __test_task (void *p) { \
    __test_result = __test_main(); \
    vTaskDelete(NULL); \
} \
static int __test_main (void)

#define K_TEST_RUN_MAIN() do { \
    k_uart_console_init(); \
    xTaskCreate(__test_task, "TESTS", configMINIMAL_STACK_SIZE * 4, NULL, 2, NULL); \
    vTaskStartScheduler(); \
    return __test_result; \
} while(0)
#else
#define K_TEST_MAIN() static int __test_main(void)
#define K_TEST_RUN_MAIN() return __test_main()
#endif

#endif
