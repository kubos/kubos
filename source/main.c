/*
 * KubOS Shell Example
 * Copyright (C) 2017 Kubos Corporation
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
#include <errno.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "FreeRTOS.h"
#include "task.h"
#include "timers.h"
#include "queue.h"

#include "kubos-hal/gpio.h"
#include "kubos-hal/led.h"
#include "kubos-hal/uart.h"

#include YOTTA_BUILD_INFO_HEADER

#include <csp/csp.h>
#include <csp/arch/csp_thread.h>
#include <slash/slash.h>

// See config.json for stack sizes etc
#include "./config.h"

static inline void blink(int pin) {
    k_led_on(pin);
    vTaskDelay(BLINK_MS / portTICK_RATE_MS);
    k_led_off(pin);
}

void csp_server(void *p) {
    (void) p;

    /* Create socket without any socket options */
    csp_socket_t *sock = csp_socket(CSP_SO_NONE);

    /* Bind all ports to socket */
    csp_bind(sock, CSP_ANY);

    /* Create 10 connections backlog queue */
    csp_listen(sock, 10);

    /* Pointer to current connection and packet */
    csp_conn_t *conn;
    csp_packet_t *packet;

    /* Process incoming connections */
    while (1) {

        /* Wait for connection, 10000 ms timeout */
        if ((conn = csp_accept(sock, 10000)) == NULL)
            continue;

        /* Read packets. Timout is 100 ms */
        while ((packet = csp_read(conn, 100)) != NULL) {
            switch (csp_conn_dport(conn)) {
                case MY_PORT:
                    /* Process packet here */
                    blink(K_LED_1);
                    csp_buffer_free(packet);
                    break;

                default:
                    /* Let the service handler reply pings, buffer use, etc. */
                    blink(K_LED_1);
                    blink(K_LED_1);
                    csp_service_handler(conn, packet);
                    break;
            }
        }

        /* Close current connection, and handle next */
        csp_close(conn);

    }
}

void task_slash(void *p) {
    struct slash *slash = slash_create(SLASH_LINE_SIZE, SLASH_HISTORY_SIZE);
    char *prompt_good = "\033[96mKubOS \033[90m%\033[0m ";
    char *prompt_bad = "\033[96mKubOS \033[31m!\033[0m ";

    slash_loop(slash, prompt_good, prompt_bad);
    slash_destroy(slash);
}

slash_command_group(led, "Control LEDs");

static inline int led_pin(const char *name) {
    for (int i = 0; i < K_LED_COUNT; i++) {
        int pin = k_led_at(i);
        const char *led_desc = k_led_get_desc(pin);

        if (strncasecmp(name, led_desc, strlen(led_desc)) == 0) {
            return pin;
        }
    }

    int index = atoi(name); // Will return 0 if undefined, which is good for us
    return k_led_at(index);
}

static int led_info(struct slash *slash)
{
    for (int i = 0; i < K_LED_COUNT; i++) {
        int led_pin = k_led_at(i);
        slash_println(slash, "LED %d: pin=%d, color=%s", i, led_pin,
                      k_led_get_desc(led_pin));
    }
    return SLASH_SUCCESS;
}
slash_command_sub(led, info, led_info, NULL, "display LED pins and colors");

static int led_on(struct slash *slash)
{
    if (slash->argc < 2) {
        return SLASH_EUSAGE;
    }

    int pin = led_pin(slash->argv[1]);
    if (pin == -1) {
        slash_println(slash, "Error: invalid pin %s", slash->argv[1]);
        return SLASH_EUSAGE;
    }

    k_led_on(pin);
    return SLASH_SUCCESS;
}
slash_command_sub(led, on, led_on, "<led>", "turn LED on");

static int led_off(struct slash *slash)
{
    if (slash->argc < 2) {
        return SLASH_EUSAGE;
    }

    int pin = led_pin(slash->argv[1]);
    if (pin == -1) {
        slash_println(slash, "Error: invalid LED %s", slash->argv[1]);
        return SLASH_EUSAGE;
    }

    k_led_off(pin);
    return SLASH_SUCCESS;
}
slash_command_sub(led, off, led_off, "<led>", "turn LED off");

static int led_blink(struct slash *slash)
{
    if (slash->argc < 2 || slash->argc > 3) {
        return SLASH_EUSAGE;
    }

    int pin = led_pin(slash->argv[1]);
    if (pin == -1) {
        slash_println(slash, "Error: invalid LED %s", slash->argv[1]);
        return SLASH_EUSAGE;
    }

    int n = 1;
    if (slash->argc > 2) {
        n = atoi(slash->argv[2]);
    }

    for (int i = 0; i < n; i++) {
        blink(pin);
        vTaskDelay(BLINK_MS / portTICK_RATE_MS);
    }

    return SLASH_SUCCESS;
}
slash_command_sub(led, blink, led_blink, "<led> [n=1]", "blink LED n times");

static int tasks(struct slash *slash)
{
    // adopted from vTaskList to go straight to stdio and not use a copy buffer
    //
    TaskStatus_t *task_status;
    UBaseType_t task_count;
    const char *status;

    task_count = uxTaskGetNumberOfTasks();
    slash_println(slash, "Number of tasks: %d", task_count);
    task_status = malloc(task_count * sizeof(TaskStatus_t));

    if (!task_status) {
        return SLASH_ENOSPC;
    }

    if (uxTaskGetSystemState(task_status, task_count, NULL) != task_count) {
        slash_println(slash, "Error getting system state");
        free(task_status);
        return SLASH_ENOSPC;
    }

    printf("# \t%-*s\tStat\tPri\tHiMrk\r\n", configMAX_TASK_NAME_LEN, "Name");
    printf("--\t%-*s\t----\t---\t-----\r\n", configMAX_TASK_NAME_LEN, "----");

    for (UBaseType_t i = 0; i < task_count; i++) {
        switch (task_status[i].eCurrentState) {
            case eReady:
                status = "RDY"; break;
            case eBlocked:
                status = "BSY"; break;
            case eSuspended:
                status = "SUS"; break;
            case eDeleted:
                status = "DEL"; break;
            default:
                status = "???"; break;
        }

        printf("%-2u\t%-*.*s\t%s\t%u\t%u\r\n",
               (unsigned int) task_status[i].xTaskNumber,
               configMAX_TASK_NAME_LEN,
               configMAX_TASK_NAME_LEN - 1,
               task_status[i].pcTaskName,
               status,
               (unsigned int) task_status[i].uxCurrentPriority,
               (unsigned int) task_status[i].usStackHighWaterMark);
    }

    free(task_status);

    return SLASH_SUCCESS;
}
slash_command(tasks, tasks, NULL, "display FreeRTOS tasks");

static int build_info(struct slash *slash)
{
    slash_println(slash, "Timestamp: %04d-%02d-%02d %02d:%02d:%02d UTC",
                  YOTTA_BUILD_YEAR, YOTTA_BUILD_MONTH, YOTTA_BUILD_DAY,
                  YOTTA_BUILD_HOUR, YOTTA_BUILD_MINUTE, YOTTA_BUILD_SECOND);

    // macro trick to stringify the generated constants
    #define _TO_STR(x) _UNWRAP(x)
    #define _UNWRAP(x) #x

    slash_println(slash, "UUID: " _TO_STR(YOTTA_BUILD_UUID));

    #ifdef YOTTA_BUILD_VCS_ID
    slash_println(slash, "VCS ID: " _TO_STR(YOTTA_BUILD_VCS_ID) ", Clean: %d",
                  YOTTA_BUILD_VCS_CLEAN);
    #endif

    return SLASH_SUCCESS;
}
slash_command(build_info, build_info, NULL, "print info about this build");

int main(void)
{
    k_uart_console_init();
    k_led_init_all();

    #ifdef TARGET_LIKE_MSP430
    /* Stop the watchdog. */
    WDTCTL = WDTPW + WDTHOLD;

    __enable_interrupt();

    P2OUT = BIT1;
    #endif

    csp_buffer_init(CSP_BUFFERS, CSP_BUFFER_SIZE);
    csp_init(MY_ADDRESS);
    csp_route_start_task(CSP_ROUTE_STACK_SIZE, 1);

    xTaskCreate(csp_server, "CSPSRV", CSP_SERVER_STACK_SIZE, NULL, 2, NULL);
    xTaskCreate(task_slash, "SLASH", SLASH_STACK_SIZE, NULL, 2, NULL);

    vTaskStartScheduler();

    // Should never get here
    k_led_on(K_LED_0);
    k_led_on(K_LED_1);

    while (1);

    return 0;
}
